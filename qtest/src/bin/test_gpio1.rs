use qtest::{gpio::Gpio, parser::Parser, socket::tcp::SocketTcp};

#[tokio::main]
async fn main() {
    let (mut parser, mut rx_irq) = Parser::<SocketTcp>::new("localhost:3000").await.unwrap();

    tokio::spawn(async move {
        loop {
            let irq = rx_irq.recv().await.unwrap();
            println!("[Parser] Received IRQ: {:?}", irq);
        }
    });

    //DIRECCIÓN DE LA GPIOC:
    const GPIOCADD: usize = 0x40020800;

    let gpio_c: Gpio = Gpio::new(GPIOCADD);

    // Inicializa el registro
    println!("Register initialized: {:?}", gpio_c.idr());

    println!("[Parser] Waiting for connection");
    parser.attach_connection().await.unwrap();
    println!("[Parser] Device connected successfully");

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido1 = gpio_c.idr().read_register(&mut parser).await;
    println!("{registro_leido1}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido2 = gpio_c.idr().read_register(&mut parser).await;
    println!("{registro_leido2}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido3 = gpio_c.idr().read_register(&mut parser).await;
    println!("{registro_leido3}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
