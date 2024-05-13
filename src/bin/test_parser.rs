use qtest_socket::parser::Parser;
use qtest_socket::socket::socket_tcp::SocketTcp;

#[tokio::main]
async fn main() {
    let (mut parser, mut rx_irq) = Parser::<SocketTcp>::new("localhost:3000").await.unwrap();

    println!("[Parser] Waiting for connection");
    parser.attach_connection().await.unwrap();
    println!("[Parser] Decive connected successfully");

    tokio::spawn(async move {
        loop {
            let irq = rx_irq.recv().await.unwrap();
            println!("[Parser] Received IRQ: {:?}", irq);
        }
    });

    {
        let res = parser.irq_intercept_in("/machine/soc").await.unwrap();
        println!("IRQ Intercept In: {:?}", res);
    }

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0)
            .await;
        println!("Set IRQ In: {:?}", res);
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    {
        let res = parser
            .set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1)
            .await;
        println!("Set IRQ In: {:?}", res);
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let res = parser.read(0, 10000).await;
    println!("Read: {:?}", res);
}
