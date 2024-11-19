use futures::{SinkExt, StreamExt};
use qtest::{parser::Parser, socket::tcp::SocketTcp, Response};
use qtest_stm32f4nucleo::Peripheral;
use std::borrow::Borrow;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::{mpsc, watch};
use tokio::sync::{mpsc::Receiver, Mutex};
use warp::reject::Reject;
use warp::ws::{Message, WebSocket};
use warp::Filter;

async fn handle_websocket(ws: WebSocket, mut ws_rx: watch::Receiver<String>) {
    let (mut tx, _) = ws.split();

    // Enviar un mensaje inicial para confirmar la conexión
    if tx
        .send(Message::text("Conexión WebSocket establecida"))
        .await
        .is_err()
    {
        eprintln!("[WebSocket] No se pudo enviar el mensaje inicial.");
        return;
    }

    // Escuchar mensajes del canal y reenviar al cliente
    while let Ok(_) = ws_rx.changed().await {
        let message = ws_rx.borrow().clone();
        if tx.send(Message::text(message)).await.is_err() {
            eprintln!("[WebSocket] Error al enviar mensaje. Cerrando conexión.");
            break;
        }
    }
}

// Define errores personalizados
#[derive(Debug)]
struct InvalidGpioName;
impl Reject for InvalidGpioName {}

#[derive(Debug)]
struct CustomError;
impl Reject for CustomError {}

// Función `set_irq_in` que manipula el estado de los pines GPIO
async fn create_interruption(
    nombre_gpio: String,
    pin: usize,
    value: isize,
    parser: &mut Parser<SocketTcp>,
) -> Result<(), warp::Rejection> {
    let gpio = match nombre_gpio.as_str() {
        "gpio_a" => "0",
        "gpio_b" => "1",
        "gpio_c" => "2",
        _ => return Err(warp::reject::custom(InvalidGpioName)),
    };

    parser
        .set_irq_in(
            &format!("/machine/soc/gpio[{}]", gpio),
            "input-in",
            pin,
            value,
        )
        .await
        .map_err(|_| warp::reject::custom(CustomError))?;
    Ok(())
}

// Filtro `pulsar_boton` que establece la ruta y recibe los parámetros
fn pulsar_boton(
    parser: Arc<Mutex<Parser<SocketTcp>>>,
    periferico: Peripheral,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "pulsar_boton" / usize / isize)
        .and(warp::any().map(move || Arc::clone(&parser)))
        .and(warp::any().map(move || periferico.clone()))
        .and_then(
            |nombre_gpio: String,
             pin: usize,
             value: isize,
             parser: Arc<Mutex<Parser<SocketTcp>>>,
             periferico: Peripheral| async move {
                let mut p = parser.lock().await;
                if let Err(e) = create_interruption(nombre_gpio, pin, value, &mut p).await {
                    return Err(e);
                }
                let register_value = periferico.gpio_c().idr().is_high(pin, &mut p).await;

                Ok::<_, warp::Rejection>(warp::reply::json(&format!(
                    "Pin {} configurado a {}. Valor del registro: {}",
                    pin, value, register_value
                )))
            },
        )
}

fn comprobar_estado(
    parser: Arc<Mutex<Parser<SocketTcp>>>,
    periferico: Peripheral,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "comprobar_estado" / usize)
        .and(warp::any().map(move || Arc::clone(&parser)))
        .and(warp::any().map(move || periferico.clone()))
        .and_then(
            |nombre_gpio: String,
             pin: usize,
             parser: Arc<Mutex<Parser<SocketTcp>>>,
             periferico: Peripheral| async move {
                let mut p = parser.lock().await;

                let register_value = periferico.gpio_a().idr().is_high(pin, &mut p).await;

                // Responde con un JSON simple
                Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({
                    "led_state": register_value
                })))
            },
        )
}

#[tokio::main]
async fn main() {
    // Inicializa el parser y el receptor de interrupciones
    let (mut parser, mut rx_irq): (Parser<SocketTcp>, Receiver<_>) =
        Parser::<SocketTcp>::new("localhost:3000").await.unwrap();

    // Inicia QEMU con los parámetros adecuados
    Command::new("../qemu_new/build/qemu-system-arm")
        .args(&[
            "-cpu",
            "cortex-m4",
            "-machine",
            "netduinoplus2",
            "-semihosting-config",
            "enable=on,target=native",
            "-monitor",
            "stdio",
            "-qtest",
            "tcp:localhost:3000",
            "-kernel",
            "../test_v1.elf",
        ])
        .spawn()
        .expect("Failed to start QEMU");

    // Crea un canal `watch` para los mensajes que serán enviados al WebSocket
    let (ws_tx, ws_rx) = watch::channel("No IRQ".to_string());

    // Procesa las IRQs y las envía al canal del WebSocket
    tokio::spawn(async move {
        loop {
            println!("[Parser] Esperando IRQ...");
            let irq = rx_irq.recv().await.unwrap();
            println!("[Parser] Received IRQ: {:?}", irq);
            if let Err(e) = ws_tx.send(format!("{:?}", irq)) {
                eprintln!("Error al enviar IRQ: {:?}", e);
            } // Envía al WebSocket
        }
    });

    // Espera a que el dispositivo se conecte
    //{
    //  let mut parser = parser.lock().await;
    parser.attach_connection().await.unwrap();

    //}
    println!("[Parser] Device connected successfully");

    let res = parser.irq_intercept_in("/machine/soc").await.unwrap();
    println!("IRQ Intercept In: {:?}", res);
    let parser = Arc::new(Mutex::new(parser)); // Usa Arc y Mutex para compartir el parser

    // {
    //     let mut parser = parser.lock().await;
    //     let res = parser.irq_intercept_in("/machine/soc").await.unwrap();
    //     println!("IRQ Intercept In: {:?}", res);
    // }

    let periferico = Peripheral::new();

    // Configuración de WebSocket
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || ws_rx.clone()))
        .map(|ws: warp::ws::Ws, ws_rx| {
            println!("Nueva conexión WebSocket aceptada.");
            ws.on_upgrade(move |socket| handle_websocket(socket, ws_rx))
        });

    // Configuración de CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);

    // Define las rutas y usa el parser en `pulsar_boton`
    let api_routes = pulsar_boton(parser.clone(), periferico.clone())
        .or(comprobar_estado(parser, periferico))
        .with(cors);

    // Combina las rutas de la API con las del WebSocket
    let routes = api_routes.or(ws_route);

    // Inicia el servidor Warp
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
