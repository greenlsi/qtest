use std::io;
use std::str;

use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

pub mod socket_tcp;
pub mod socket_unix;

pub trait Socket {
    fn new(url: &str, out_handler: mpsc::Sender<String>) -> impl std::future::Future<Output = io::Result<Self>> + Send
    where
        Self: Sized;

    fn attach_connection(&mut self) -> impl std::future::Future<Output = io::Result<()>> + Send;
    fn send(&mut self, data: &str) -> impl std::future::Future<Output = io::Result<usize>> + Send;

    fn address(&self) -> String;
    fn close(&self) -> io::Result<()>;

    fn reader<T: AsyncReadExt + Unpin + Send>(
        mut owned_read_half: T,
        out_handler: mpsc::Sender<String>,
    ) -> impl std::future::Future<Output = ()> + Send {async move {
        let mut buf = [0; 1024];
        loop {
            let mut msg = String::new();

            while !msg.contains('\n') {
                buf.fill(0);

                let msg_part = match owned_read_half.read(&mut buf).await {
                    Ok(0) => {
                        println!("[QTEST_SOCKET] Connection closed by peer");
                        return;
                    }
                    Ok(_) => str::from_utf8(&buf).unwrap().to_string(),
                    Err(e) => {
                        println!("[QTEST_SOCKET] [ERROR] read error: {:?}", e);
                        break;
                    }
                };

                msg.push_str(&msg_part);
            }

            out_handler.send(msg).await.unwrap();
        }
    } }
}
