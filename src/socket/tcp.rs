use std::io;

use tokio::{
    io::AsyncWriteExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::mpsc,
};

use super::{reader, Socket};

/// This struct should be used to interact with QEMU using a tcp socket via [crate::parser::Parser] struct.
#[derive(Debug)]
pub struct SocketTcp {
    socket: TcpListener,

    out_handler: mpsc::Sender<String>,

    write_stream: Option<OwnedWriteHalf>,
}

impl Socket for SocketTcp {
    async fn new(url: &str, out_handler: mpsc::Sender<String>) -> io::Result<Self> {
        match TcpListener::bind(url).await {
            Ok(socket) => Ok(Self {
                socket,
                out_handler,
                write_stream: None,
            }),
            Err(e) => Err(e),
        }
    }

    async fn attach_connection(&mut self) -> io::Result<()> {
        match self.socket.accept().await {
            Ok((stream, _)) => {
                let (read_stream, write_stream) = stream.into_split();
                self.write_stream = Some(write_stream);
                let cloned_out_handler = self.out_handler.clone();
                tokio::spawn(async move {
                    reader::<OwnedReadHalf>(read_stream, cloned_out_handler).await;
                });
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn address(&self) -> String {
        let addr = self.socket.local_addr().unwrap();
        format!("{}:{}", addr.ip(), addr.port())
    }

    fn close(&self) -> io::Result<()> {
        Ok(())
    }

    async fn send(&mut self, data: &str) -> io::Result<usize> {
        match self.write_stream.as_mut() {
            Some(stream) => stream.write(data.as_bytes()).await,
            None => Err(io::Error::new(io::ErrorKind::NotConnected, "No connection")),
        }
    }
}
