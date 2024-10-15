use qtest::{parser::Parser, register::Register, socket::tcp::SocketTcp};

#[tokio::main]
async fn main() {
    // let (mut parser, mut rx_irq) = Parser::<SocketTcp>::new("localhost:3000").await.unwrap();

    // println!("[Parser] Waiting for connection");
    // parser.attach_connection().await.unwrap();
    // println!("[Parser] Device connected successfully");

    //creo que la dirección del gpioC 13 0x4002 0810
    let name = "MyRegister";
    let address = 0x40020810;
    let size = 32; // Por ejemplo, tamaño en bits
    let url = "localhost:3000";

    // Inicializa el registro
    let mut register: Register<u32, SocketTcp> =
        Register::new(name, address, size, url).await.unwrap();
    println!("Register initialized: {:?}", register);

    println!("[Parser] Waiting for connection");
    register.get_parser().attach_connection().await.unwrap();
    println!("[Parser] Device connected successfully");

    {
        let res = register
            .get_parser()
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido1 = register.read_register().await;
    println!("{registro_leido1}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = register
            .get_parser()
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido2 = register.read_register().await;
    println!("{registro_leido2}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = register
            .get_parser()
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }
    let registro_leido3 = register.read_register().await;
    println!("{registro_leido3}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
