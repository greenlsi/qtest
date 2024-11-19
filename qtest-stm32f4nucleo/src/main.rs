use warp::{Filter, reject::Reject};
use std::{process::Command, sync::Arc};
use tokio::sync::{Mutex, mpsc::Receiver};
use qtest::{parser::Parser, socket::tcp::SocketTcp, Response};
// Define errores personalizados
#[derive(Debug)]
struct InvalidGpioName;
impl Reject for InvalidGpioName {}

#[derive(Debug)]
struct CustomError;
impl Reject for CustomError {}

// Función `set_irq_in` que manipula el estado de los pines GPIO
async fn set_irq_in(
    nombre_gpio: String,
    pin: usize,
    value: isize,
    parser: Arc<Mutex<Parser<SocketTcp>>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Mapeo de nombres de GPIO a índices de hardware
    let gpio = match nombre_gpio.as_str() {
        "gpio_a" => "0",
        "gpio_b" => "1",
        "gpio_c" => "2",
        _ => return Err(warp::reject::custom(InvalidGpioName)),
    };

    // Acceso al parser y configuración del pin
    let mut parser = parser.lock().await;
    let res = parser
        .set_irq_in(
            &format!("/machine/soc/gpio[{}]", gpio),
            "input-in",
            pin,
            value,
        )
        .await;

    match res {
        Ok(_) => Ok(warp::reply::json(&format!("Pin {} set to {}", pin, value))),
        Err(_) => Err(warp::reject::custom(CustomError)),
    }
}

// Filtro `pulsar_boton` que establece la ruta y recibe los parámetros
fn pulsar_boton(
    parser: Arc<Mutex<Parser<SocketTcp>>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!(String / "pulsar_boton" / usize / isize)
        .and(warp::any().map(move || Arc::clone(&parser)))
        .and_then(|nombre_gpio: String, pin: usize, value: isize, parser| async move {
            set_irq_in(nombre_gpio, pin, value, parser).await
        })
}

// Ruta de ejemplo `message_route`
fn message_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api")
        .and(warp::path("message"))
        .map(|| "Este es el mensaje desde el backend!")
}

#[tokio::main]
async fn main() {
    // Inicializa el parser y el receptor de interrupciones
    let (parser, mut rx_irq): (Parser<SocketTcp>, Receiver<_>) = Parser::<SocketTcp>::new("localhost:3000").await.unwrap();
    let parser = Arc::new(Mutex::new(parser)); // Usa Arc y Mutex para compartir el parser

    // Inicia un hilo que recibirá IRQs de manera asincrónica
    let parser_for_thread = Arc::clone(&parser);
    tokio::spawn(async move {
        while let Some(irq) = rx_irq.recv().await {
            println!("[Parser] Received IRQ: {:?}", irq);
        }
    });

    // Inicia QEMU con los parámetros adecuados
    Command::new("../qemu_new/build/qemu-system-arm")
        .args(&[
            "-cpu", "cortex-m4",
            "-machine", "netduinoplus2",
            "-semihosting-config", "enable=on,target=native",
            "-monitor", "stdio",
            "-qtest", "tcp:localhost:3000",
            "-kernel", "./bin/test_v1.elf",
        ])
        .spawn()
        .expect("Failed to start QEMU");

    // Espera a que el dispositivo se conecte
    {
        let mut parser = parser.lock().await;
        parser.attach_connection().await.unwrap();
    }
    println!("[Parser] Device connected successfully");

    // Configuración de CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);

    // Define las rutas y usa el parser en `pulsar_boton`
    let routes = message_route().or(pulsar_boton(parser)).with(cors);

    // Inicia el servidor Warp
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
