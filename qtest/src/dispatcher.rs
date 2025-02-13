use crate::{socket::SocketReader, Irq, Response};
use tokio::{
    io::{Error, ErrorKind, Result},
    sync::mpsc::Sender,
    task::JoinHandle,
};

/// Used to read data from the QTest socket.
pub(crate) struct Dispatcher {
    /// Reader half of the TCP socket connected to QTest.
    socket_reader: SocketReader,
    /// Sender for [`Response`] data.
    responser_sender: Sender<Response>,
    /// Sender for [`Irq`] data.
    irq_sender: Sender<Irq>,
}

impl Dispatcher {
    /// Create a new reader instance with the given receivers and senders
    pub(crate) fn new(
        socket_reader: SocketReader,
        responser_sender: Sender<Response>,
        irq_sender: Sender<Irq>,
    ) -> Self {
        Self {
            socket_reader,
            responser_sender,
            irq_sender,
        }
    }

    /// Reads data from the socket and sends it to the IRQ or Response channels
    async fn read(&mut self) -> Result<()> {
        loop {
            let raw_data = self.socket_reader.read().await?;
            let str_data = raw_data.trim_matches(char::from(0)); // Remove null characters
            for line in str_data.lines() {
                if line.is_empty() {
                    continue; // Skip empty lines
                }

                match Irq::try_from(line) {
                    Ok(irq) => self.irq_sender.send(irq).await.map_err(|e| {
                        Error::new(ErrorKind::Other, format!("Could not send IRQ: {e}"))
                    }),
                    Err(_) => self
                        .responser_sender
                        .send(Response::from(line))
                        .await
                        .map_err(|e| {
                            Error::new(ErrorKind::Other, format!("Could not send response: {e}"))
                        }),
                }?;
            }
        }
    }

    pub(crate) fn spawn(mut self) -> JoinHandle<Result<()>> {
        tokio::spawn(async move { self.read().await })
    }
}
