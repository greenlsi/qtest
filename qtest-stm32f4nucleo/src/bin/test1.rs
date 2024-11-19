use qtest::{parser::Parser, register::Register, socket::tcp::SocketTcp};
use std::process::Command;

#[tokio::main]
async fn main() {
    let (mut parser, mut rx_irq) = Parser::<SocketTcp>::new("localhost:3000").await.unwrap();

    tokio::spawn(async move {
        loop {
            let irq = rx_irq.recv().await.unwrap();
            println!("[Parser] Received IRQ: {:?}", irq);
        }
    });

    //creo que la dirección del idr para gpioC es 0x4002 0810, nos fijaremos en el pin 13 en concreto
    let address = 0x40020810;
    let name = "registro";

    // Inicializa el registro
    let gpioc_idr: Register<u32> = Register::new(name, address);
    println!("Register initialized: {:?}", gpioc_idr);

    println!("[Parser] Waiting for connection");

    //Starting QEMU:
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

    parser.attach_connection().await.unwrap();
    println!("[Parser] Device connected successfully");

    // {
    //     let res = parser
    //         .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
    //         .await;
    //     println!("Set IRQ In: {:?}", res);
    // }
    // let pin_leido1 = gpioc_idr.read_pin(13, &mut parser).await;
    // let registro_leido1 = gpioc_idr.read_register(&mut parser).await;
    // println!("{registro_leido1}, {pin_leido1}");
    // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido2 = gpioc_idr.read_register(&mut parser).await;
    println!("{registro_leido2}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
            .await;
        println!("Set IRQ In: {:?}", res);
    }

    let registro_leido3 = gpioc_idr.read_register(&mut parser).await;
    println!("{registro_leido3}");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }

    let registro_leido4 = gpioc_idr.read_register(&mut parser).await;
    println!("{registro_leido4}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
