use std::io;

use tokio::{io::AsyncWriteExt, net::{tcp::{OwnedReadHalf, OwnedWriteHalf}, TcpListener}, sync::mpsc};

use crate::qsocket::qtest_socket::QTestSocket;

pub struct QTestSocketTcp {
    socket: TcpListener,

    out_handler: mpsc::Sender<String>,

    write_stream: Option<OwnedWriteHalf>
}

impl QTestSocketTcp {
    pub async fn new(url: &str, out_handler: mpsc::Sender<String>) -> io::Result<Self> {
        match TcpListener::bind(url).await {
            Ok(socket) => Ok(Self {
                socket,
                out_handler,
                write_stream: None
            }),
            Err(e) => Err(e)
        }
    }
}

impl QTestSocket for QTestSocketTcp {
    async fn attach_connection(&mut self) -> io::Result<()> {
        match self.socket.accept().await {
            Ok((stream, _)) => {
                let (read_stream, write_stream) = stream.into_split();
                self.write_stream = Some(write_stream);
                let cloned_out_handler = self.out_handler.clone();
                tokio::spawn(async move {
                    <QTestSocketTcp as QTestSocket>::reader::<OwnedReadHalf>(read_stream, cloned_out_handler).await;
                });
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    fn address(&self) -> String {
        let addr = self.socket.local_addr().unwrap().ip();
        let port = self.socket.local_addr().unwrap().port();

        format!("{}:{}", addr, port)
    }

    async fn send(&mut self, data: &str) -> io::Result<usize> {
        match self.write_stream.as_mut() {
            Some(stream) => stream.write(data.as_bytes()).await,
            None => Err(io::Error::new(io::ErrorKind::NotConnected, "No connection"))
        }
    }
}