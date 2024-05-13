use std::io;

use qtest_socket::socket::{socket_tcp::SocketTcp, Socket};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let url = "localhost:3000";
    let (tx_sock_out, mut rx_sock_out) = mpsc::channel(32);
    //    let (tx_sock_in, rx_sock_in) = mpsc::channel(32);

    let mut qtest_socket = SocketTcp::new(url, tx_sock_out /*, rx_sock_in*/)
        .await
        .unwrap();

    println!("QTestSocket listening @ {}", qtest_socket.address());

    qtest_socket.attach_connection().await.unwrap();

    println!("Qemu attached");

    tokio::spawn(async move {
        println!("Started listening thread");
        while let Some(msg) = rx_sock_out.recv().await {
            print!("{}", msg);
        }
    });

    loop {
        let mut in_buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut in_buffer).unwrap();

        match in_buffer.trim() {
            "exit" => {
                println!("Exiting");
                break;
            }
            _ => {
                qtest_socket.send(&in_buffer).await.unwrap();
            }
        }
    }
}
