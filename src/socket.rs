use std::{io, str};
use tokio::{io::AsyncReadExt, sync::mpsc};

pub mod tcp;
pub mod unix;

/// Interface for the socket implementations.
pub trait Socket {
    /// Creates a new socket instance.
    ///
    /// This method should be used to create a new socket instance. The `out_handler` parameter
    /// is a Tokio MPSC channel sender that will be used to send messages to the parser.
    fn new(
        url: &str,
        out_handler: mpsc::Sender<String>,
    ) -> impl std::future::Future<Output = io::Result<Self>> + Send
    where
        Self: Sized;

    /// Attaches a connection to the socket.
    ///
    /// The [`send`] and [`receive`] methods will not work until this method is called.
    fn attach_connection(&mut self) -> impl std::future::Future<Output = io::Result<()>> + Send;

    /// Sends a message to the socket and returns the size of the message sent.
    ///
    /// # Note
    ///
    /// QTest uses a newline character to delimit messages and will not start parsing the message until it receives it.
    ///
    /// This method will not work before calling [`attach_connection`].
    fn send(&mut self, data: &str) -> impl std::future::Future<Output = io::Result<usize>> + Send;

    /// Returns the address of the socket.
    fn address(&self) -> String;

    /// Closes the socket.
    fn close(&self) -> io::Result<()>;
}

/// Reads messages from the socket. Returns Err if the connection was closed by peer or an error occurred.
///
/// The messages are sent to the `out_handler` channel that was passed to the new method.
async fn reader<T: AsyncReadExt + Unpin + Send>(
    mut owned_read_half: T,
    out_handler: mpsc::Sender<String>,
) {
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
}
