use std::{fs, io};

use tokio::{
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf},
        UnixListener,
    },
    sync::mpsc,
};

use super::{reader, Socket};

/// This struct should be used to interact with QEMU using a UNIX socket via [crate::parser::Parser] struct.
pub struct SocketUnix {
    socket: UnixListener,
    out_handler: mpsc::Sender<String>,
    write_stream: Option<OwnedWriteHalf>,
    path: String,
}

impl Socket for SocketUnix {
    async fn new(path: &str, out_handler: mpsc::Sender<String>) -> io::Result<Self> {
        match UnixListener::bind(path) {
            Ok(socket) => Ok(Self {
                socket,
                out_handler,
                write_stream: None,
                path: path.to_string(),
            }),
            Err(e) => match e.kind() {
                io::ErrorKind::AddrInUse => {
                    match fs::remove_file(path) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    match UnixListener::bind(path) {
                        Ok(socket) => Ok(Self {
                            socket,
                            out_handler,
                            write_stream: None,
                            path: path.to_string(),
                        }),
                        Err(e) => Err(e),
                    }
                }
                _ => Err(e),
            },
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
        self.path.clone()
    }

    fn close(&self) -> io::Result<()> {
        fs::remove_file(self.path.clone())
    }

    async fn send(&mut self, data: &str) -> io::Result<usize> {
        match self.write_stream.as_mut() {
            Some(stream) => stream.try_write(data.as_bytes()),
            None => Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "No connection attached",
            )),
        }
    }
}
