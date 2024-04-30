
use std::io;
use std::str;

use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;


pub trait QTestSocket {

    async fn attach_connection(&mut self) -> io::Result<()>;
    
    fn address(&self) -> String;
    async fn send(&mut self, data: &str) -> io::Result<usize>;

    async fn reader<T: AsyncReadExt + Unpin>(mut owned_read_half: T, out_handler: mpsc::Sender<String>) {
        let mut buf = [0; 1024];
        loop {

            buf.fill(0);

            let msg = match owned_read_half.read(&mut buf).await {
                Ok(0) => {
                    println!("[QTEST_SOCKET] Connection closed by peer");
                    return;
                }
                Ok(_) => {
                    str::from_utf8(&buf).unwrap().to_string()
                }
                Err(e) => {
                    println!("[QTEST_SOCKET] [ERROR] read error: {:?}", e);
                    break;
                }
            };

            out_handler.send(msg).await.unwrap();
        }

    }
}