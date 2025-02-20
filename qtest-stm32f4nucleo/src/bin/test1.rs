use qtest::{parser::Parser, register::Register, socket::tcp::SocketTcp};
//use std::process::Command;
use tokio::time::{sleep, Duration};
use tracing::{ error, info, warn};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Inicializa tracing con un formato de salida básico
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // Muestra logs de nivel DEBUG o superior
        .init();

    let address = 0x40020810; // Dirección del registro GPIOC IDR
    let name = "GpioC_IDR";

    // Inicializa el parser y el canal de interrupciones
    let (mut parser, mut rx_irq) = match Parser::<SocketTcp>::new("localhost:3000").await {
        Ok(p) => p,
        Err(e) => {
            error!("Fallo al inicializar el parser: {:?}", e);
            return;
        }
    };

    // Inicializa el registro
    let gpioc_idr: Register<u32> = Register::new(name, address);
    info!("Registro inicializado: {:?}", gpioc_idr);

    info!("Esperando conexión del dispositivo...");

    // // Inicia QEMU y maneja posibles errores
    // if let Err(e) = Command::new("../qemu_new/build/qemu-system-arm")
    //     .args([
    //         "-cpu",
    //         "cortex-m4",
    //         "-machine",
    //         "netduinoplus2",
    //         "-semihosting-config",
    //         "enable=on,target=native",
    //         "-monitor",
    //         "stdio",
    //         "-qtest",
    //         "tcp:localhost:3000",
    //         "-kernel",
    //         "../test_v1.elf",
    //     ])
    //     .spawn()
    // {
    //     error!("No se pudo iniciar QEMU: {:?}", e);
    //     return;
    // }

    // Intenta conectar con el dispositivo
    if let Err(e) = parser.attach_connection().await {
        error!("Fallo al conectar con el dispositivo: {:?}", e);
        return;
    }
    info!("Conexión con el dispositivo exitosa.");

    // Intenta interceptar IRQs
    if let Err(e) = parser.irq_intercept_in("/machine/soc").await {
        error!("Fallo al registrar IRQ intercept: {:?}", e);
        return;
    }
    info!("Interceptación de IRQ configurada con éxito.");

    // Procesador de interrupciones (en un hilo separado)
    let irq_handler = tokio::spawn(async move {
        while let Some(irq) = rx_irq.recv().await {
            info!("[IRQ] Recibida: {:?}", irq);
        }
        warn!("[IRQ] Canal cerrado.");
    });

    // Lógica principal del programa
    loop {
        info!("Ciclo principal: escribiendo al registro.");

        if let Err(e) = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await
        {
            error!("Fallo al establecer IRQ (input-in 1): {:?}", e);
            break;
        }

        let registro_leido2 = gpioc_idr.read_register(&mut parser).await.unwrap();
        println!("{registro_leido2}");

        sleep(Duration::from_secs(3)).await;

        if let Err(e) = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
            .await
        {
            error!("Fallo al establecer IRQ (input-in 0): {:?}", e);
            break;
        }

        let registro_leido3 = gpioc_idr.read_register(&mut parser).await.unwrap();
        println!("{registro_leido3}");

        sleep(Duration::from_secs(3)).await;
    }

    // Espera al cierre del manejador de interrupciones
    if let Err(e) = irq_handler.await {
        error!("Fallo en el manejador de IRQs: {:?}", e);
    }

    info!("Programa finalizado.");
}
