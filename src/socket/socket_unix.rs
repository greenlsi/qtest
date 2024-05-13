use std::{fs, io};

use tokio::{
    net::{
        unix::{OwnedReadHalf, OwnedWriteHalf},
        UnixListener,
    },
    sync::mpsc,
};

use super::socket::Socket;

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
                            println!("[QTEST_SOCKET_UNIX] [ERROR] Failed to removed sockeet file; err = {:?}", e);
                            return Err(e);
                        }
                    };

                    return match UnixListener::bind(path) {
                        Ok(socket) => Ok(Self {
                            socket,
                            out_handler,
                            write_stream: None,
                            path: path.to_string(),
                        }),
                        Err(e) => Err(e),
                    };
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
                    <SocketUnix as Socket>::reader::<OwnedReadHalf>(
                        read_stream,
                        cloned_out_handler,
                    )
                    .await;
                });
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn address(&self) -> String {
        String::from(self.path.to_string())
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
