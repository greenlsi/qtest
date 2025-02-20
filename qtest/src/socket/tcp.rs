use super::{reader, Socket};
use std::io;
use tokio::{
    io::AsyncWriteExt,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    sync::mpsc,
};

/// This struct should be used to interact with QEMU using a TCP socket via [crate::parser::Parser] struct.
#[derive(Debug)]
pub struct SocketTcp {
    /// The TCP listener instance.
    socket: TcpListener,
    /// Queue to send messages to the parser.
    out_handler: mpsc::Sender<String>,
    /// The write stream to send messages to the client. This is set after calling `attach_connection`.
    write_stream: Option<OwnedWriteHalf>,
}

impl Socket for SocketTcp {
    /// Creates a new `SocketTcp` instance.
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

    /// Attaches a connection to the socket.
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

    /// Returns the address of the socket.
    fn address(&self) -> String {
        let addr = self.socket.local_addr().unwrap();
        format!("{}:{}", addr.ip(), addr.port())
    }

    /// Closes the socket.
    fn close(&self) -> io::Result<()> {
        Ok(())
    }

    /// Sends a message to the socket and returns the size of the message sent.
    async fn send(&mut self, data: &str) -> io::Result<usize> {
        match self.write_stream.as_mut() {
            Some(stream) => stream.write(data.as_bytes()).await,
            None => Err(io::Error::new(io::ErrorKind::NotConnected, "No connection")),
        }
    }
}
