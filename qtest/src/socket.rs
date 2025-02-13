use crate::session::Session;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Error, ErrorKind, Result},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
};

/// QTest socket listener. It acts as a server listening for incoming connections.
#[repr(transparent)]
#[derive(Debug)]
pub struct SocketListener {
    /// The inner TCP listener instance.
    socket: TcpListener,
}

impl SocketListener {
    /// Creates a new TCP listener ready to accept connections.
    /// The `url` parameter specifies the address and port to bind the listener to.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qtest::socket::SocketListener;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut listener = SocketListener::new("localhost:3000").await.unwrap();
    ///     println!("Socket listener ready. listening on {}", listener.address());
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the listener could not be created (e.g. if the address is already in use).
    pub async fn new(url: &str) -> Result<Self> {
        TcpListener::bind(url).await.map(|socket| Self { socket })
    }

    /// Returns the address of the socket.
    pub fn address(&self) -> String {
        let addr = self.socket.local_addr().unwrap();
        format!("{}:{}", addr.ip(), addr.port())
    }

    /// Establishes a new TCP connection and returns the reader and the writer halves of the connection.
    async fn connect(&mut self) -> Result<(SocketReader, SocketWriter)> {
        let (stream, _) = self.socket.accept().await?;
        let (read_stream, write_stream) = stream.into_split();
        Ok((
            SocketReader::new(read_stream),
            SocketWriter::new(write_stream),
        ))
    }

    /// Establishes a new QTest session and returns the session proxy and the IRQ queue receiver.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use qtest::socket::SocketListener;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let mut listener = SocketListener::new("localhost:3000").await.unwrap();
    ///    let session = listener.new_session().await.unwrap();
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// You can create as many sessions as you want from a single listener.
    pub async fn new_session(&mut self) -> Result<Session> {
        let (socket_reader, socket_writer) = self.connect().await?;
        Ok(Session::new(socket_reader, socket_writer))
    }
}

/// QTest socket reader. It reads data from the TCP ocket.
#[derive(Debug)]
pub(crate) struct SocketReader {
    /// Reader half of the TCP socket.
    read_stream: OwnedReadHalf,
    /// Buffer to store the read data.
    buf: [u8; 1024],
}

impl SocketReader {
    /// Creates a new socket reader instance.
    const fn new(read_stream: OwnedReadHalf) -> Self {
        Self {
            read_stream,
            buf: [0; 1024],
        }
    }

    /// Reads data from QEMU through the socket and returns it as a string.
    pub(crate) async fn read(&mut self) -> Result<String> {
        let mut msg = String::new();
        while !msg.contains('\n') {
            self.buf.fill(0); // Clear the buffer
            let msg_part = match self.read_stream.read(&mut self.buf).await? {
                0 => {
                    return Err(Error::new(
                        ErrorKind::ConnectionAborted,
                        "Connection closed by peer",
                    ));
                }
                _ => std::str::from_utf8(&self.buf).unwrap().to_string(),
            };
            msg.push_str(&msg_part);
        }
        Ok(msg)
    }
}

/// QTest socket writer. It writes data to the TCP socket.
#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct SocketWriter {
    /// Writer half of the TCP socket.
    write_stream: OwnedWriteHalf,
}

impl SocketWriter {
    /// Creates a new socket writer instance.
    const fn new(write_stream: OwnedWriteHalf) -> Self {
        Self { write_stream }
    }

    /// Sends a string to QEMU through the socket.
    pub(crate) async fn write(&mut self, data: &str) -> Result<usize> {
        self.write_stream.write(data.as_bytes()).await
    }
}
