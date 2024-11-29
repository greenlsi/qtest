use qtest::{parser::Parser, socket::tcp::SocketTcp, Response};
use qtest_stm32f4nucleo::Peripheral;
use std::{process::Command, sync::Arc};
use tokio::sync::{mpsc::Receiver, Mutex};
use warp::{reject::Reject, Filter};
use serde_json;

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

    // Espera a que el dispositivo se conecte

    parser.attach_connection().await.unwrap();
   
    println!("[Parser] Device connected successfully");
    let res = parser.irq_intercept_in("/machine/soc").await.unwrap();
    println!("IRQ Intercept In: {:?}", res);

    let parser = Arc::new(Mutex::new(parser)); // Usa Arc y Mutex para compartir el parser
        // Inicia un hilo que recibirá IRQs de manera asincrónica
        tokio::spawn(async move {
            while let Some(irq) = rx_irq.recv().await {
                println!("[Parser] Received IRQ: {:?}", irq);
            }
        });

    let periferico = Peripheral::new();

    // Configuración de CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["Content-Type"]);

    // Define las rutas y usa el parser en `pulsar_boton`
    let routes = pulsar_boton(parser.clone(), periferico.clone())
        .or(comprobar_estado(parser, periferico))
        .with(cors);

    // Inicia el servidor Warp
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
