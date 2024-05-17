use std::io;
use std::str;

use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

pub mod socket_tcp;
pub mod socket_unix;

/// Socket trait, used to define the interface for the socket implementations
///
/// Socket implementations must implement this trait to be used by the parser.
pub trait Socket {
    /// Create a new socket instance
    ///
    /// This method should be used to create a new socket instance. The out_handler parameter is a tokio mpsc channel sender that will be used to send messages to the parser.
    /// Both send and receive methods will not work until the attach_connection method is called. The rx parameter shold be handled with another tokio task.
    ///
    /// # Example
    ///
    /// In this example we create a tcp socket instance that implements the Socket trait.
    ///
    /// ```
    /// let (tx, rx) = mpsc::channel(32);
    ///
    /// let socket = SocketTcp::new("localhost:3000", tx).await.unwrap();
    ///
    /// // Accept qtest connection
    /// socket.attach_connection().await.unwrap();
    ///
    /// // Handle incoming messages
    /// tokio::spawn(async move {
    ///     while let Some(msg) = rx.recv().await {
    ///         // Handle messages...    
    ///     }
    /// });
    ///
    /// // Send messages to qtest...
    /// ```
    fn new(
        url: &str,
        out_handler: mpsc::Sender<String>,
    ) -> impl std::future::Future<Output = io::Result<Self>> + Send
    where
        Self: Sized;

    /// Attach a connection to the socket
    ///
    /// This method should be called after the socket is created to establish a connection.
    /// The send and receve methods will not work until this method is called.
    ///
    /// # Example
    ///
    /// In this example we create a tcp socket instance that implements the Socket trait and attach a connection to it.
    ///
    /// ```
    ///  let (tx, rx) = mpsc::channel(32);
    ///
    ///  let socket = SocketTcp::new("localhost:3000", tx).await.unwrap();
    ///
    ///  // Accept qtest connection
    ///  socket.attach_connection().await.unwrap();
    ///
    ///  // Handle incoming messages
    ///  tokio::spawn(async move {
    ///      while let Some(msg) = rx.recv().await {
    ///          // Handle messages...    
    ///      }
    ///  });
    ///
    ///  // Send messages to qtest...
    /// ```
    fn attach_connection(&mut self) -> impl std::future::Future<Output = io::Result<()>> + Send;

    /// Send a message to the socket. Returns the size of the message sent.
    ///
    /// This method should be used to send messages to the socket. The message should be a string, keep in mind that qtest
    /// uses a newline character to delimit messages, and wont start parsing the message until it receives it.
    ///
    /// **Important:** This method wont work until a connection is attached to the socket.
    fn send(&mut self, data: &str) -> impl std::future::Future<Output = io::Result<usize>> + Send;

    /// Returns the addres of the socket as a String
    fn address(&self) -> String;

    /// Close the socket. Returns an io::Result with the result of the operation.
    fn close(&self) -> io::Result<()>;

    /// Read messages from the socket. Only returns Err if the connection was closed by peer or an error occurred.
    ///
    /// This method should be used to read messages from the socket.
    /// The messages are sent to the out_handler channel that was passed to the new method.
    ///
    /// This method should be handled in another tokio task.
    ///
    /// **Important:** This method shouldn't be used directly by the end user unless they want to create another socket implementation,
    /// it's used internally by the socket implementations.
    fn reader<T: AsyncReadExt + Unpin + Send>(
        mut owned_read_half: T,
        out_handler: mpsc::Sender<String>,
    ) -> impl std::future::Future<Output = ()> + Send {
        async move {
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
    }
}
