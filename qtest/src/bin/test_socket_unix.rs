use qtest::socket::{unix::SocketUnix, Socket};
use std::io;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let url = "/tmp/gpio.sock";
    let (tx_sock_out, mut rx_sock_out) = mpsc::channel(32);

    let mut qtest_socket = SocketUnix::new(url, tx_sock_out).await.unwrap();

    println!("qtest unix socket listening @ {}", qtest_socket.address());

    qtest_socket.attach_connection().await.unwrap();

    println!("QEMU attached");

    tokio::spawn(async move {
        println!("Started listening thread");
        while let Some(msg) = rx_sock_out.recv().await {
            print!("{msg}");
        }
    });

    loop {
        let mut in_buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut in_buffer).unwrap();

        match in_buffer.trim() {
            "exit" => {
                println!("Closing server");
                qtest_socket.close().unwrap();
                return;
            }
            _ => {
                qtest_socket.send(&in_buffer).await.unwrap();
            }
        }
    }
}
