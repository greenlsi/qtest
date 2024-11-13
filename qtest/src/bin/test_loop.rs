use qtest::{parser::Parser, socket::tcp::SocketTcp};
use std::time::SystemTime;

#[tokio::main]
async fn main() {
    let (mut parser, mut rx_irq) = Parser::<SocketTcp>::new("localhost:3000").await.unwrap();

    println!("[Parser] Waiting for connection");
    parser.attach_connection().await.unwrap();
    println!("[Parser] Device connected successfully");

    tokio::spawn(async move {
        loop {
            let irq = rx_irq.recv().await.unwrap();
            println!("[Parser] Received IRQ: {:?}", irq);
        }
    });

    
        let res = parser.irq_intercept_in("/machine/soc/gpio[2]").await.unwrap();
        println!("IRQ Intercept In: {:?}", res);
    

     
        parser.set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    loop {
        let mut time = SystemTime::now();
        println!("inicial{:?}",time);

        parser.set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 0).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        time = SystemTime::now();
        println!("medio{:?}",time);

        parser.set_irq_in("/machine/soc/gpio[2]", "input-in", 13, 1).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        time = SystemTime::now();
        println!("final{:?}",time);

    }

}