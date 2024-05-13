
use std::io;
use std::str;

use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

pub trait Socket {

    async fn new(url: &str, out_handler: mpsc::Sender<String>) -> io::Result<Self> where Self: Sized;

    async fn attach_connection(&mut self) -> io::Result<()>;
    async fn send(&mut self, data: &str) -> io::Result<usize>;
    
    fn address(&self) -> String;
    fn close(&self) -> io::Result<()>;

    async fn reader<T: AsyncReadExt + Unpin>(mut owned_read_half: T, out_handler: mpsc::Sender<String>) {
        let mut buf = [0; 1024];
        loop {

            let mut msg = String::new();

            while !msg.contains("\n") {
                buf.fill(0);

                let msg_part = match owned_read_half.read(&mut buf).await {
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

                msg.push_str(&msg_part);
            }

            out_handler.send(msg).await.unwrap();
        }

    }
}