use base64::{
    alphabet,
    engine::{Engine, GeneralPurpose, GeneralPurposeConfig},
};
use std::io;
use tokio::sync::mpsc;

use crate::socket::Socket;
use crate::{Irq, Response};

const ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, GeneralPurposeConfig::new());

/// Parser struct, used to interact with qtest
#[derive(Debug)]
pub struct Parser<T: Socket> {
    socket: T,
    response_queue: mpsc::Receiver<Response>,
}

impl<T: Socket> Parser<T> {
    /// Create a new parser instance, with the given URL and specific socket implementation.
    ///
    /// Returns a result with the parser instance and a receiver for IRQs.
    /// The IRQ receiver should be managed by the user with the `recv` method.
    /// The parser will not work until the channel is managed and the method `attach_connection` is called,
    /// in order to attach the parser to the QTest socket connection.
    ///
    /// # Example
    ///
    /// ```
    /// let (parser, irq_rx) = Parser::<TcpSocket>::new("localhost:3000").await.unwrap();
    ///
    /// parser.attach_connection().await.unwrap();
    ///
    /// tokio::spawn(async move {
    ///    while let Some(irq) = irq_rx.recv().await {
    ///       println!("IRQ: {:?}", irq);
    ///   }
    /// });
    /// ```
    pub async fn new(url: &str) -> io::Result<(Parser<T>, mpsc::Receiver<Irq>)> {
        let (tx_raw_sock_out, rx_raw_sock_out) = mpsc::channel(32);
        let (tx_response, rx_response) = mpsc::channel(32);
        let (tx_irq, rx_irq) = mpsc::channel(32);

        let qtest_socket = T::new(url, tx_raw_sock_out).await?;

        tokio::spawn(async move {
            let mut reader = Reader::new(rx_raw_sock_out, tx_irq, tx_response);
            reader.read().await.unwrap();
        });

        Ok((
            Parser {
                socket: qtest_socket,
                response_queue: rx_response,
            },
            rx_irq,
        ))
    }

    pub async fn attach_connection(&mut self) -> io::Result<()> {
        self.socket.attach_connection().await
    }

    /// Clock step function, steps the clock by the given number of nanoseconds
    pub async fn clock_step(&mut self, ns: Option<usize>) -> io::Result<Response> {
        let data = match ns {
            Some(ns) => format!("clock_step {ns}\n"),
            None => "clock_step\n".to_string(),
        };
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

    /// Set the clock to the given number of nanoseconds
    pub async fn clock_set(&mut self, ns: usize) -> io::Result<usize> {
        let data = format!("clock_set {}\n", ns);
        self.socket.send(&data).await?;
        let response =
            self.response_queue.recv().await.ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "Could not receive response")
            })?;

        match response {
            Response::OkVal(val) => val.parse().map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Could not parse value: {}\n error {}", val, e),
                )
            }),
            Response::Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("invalid response: {}", e),
            )),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid response")),
        }
    }

    /// IRQ intercept in function, intercepts the given IRQ in the given QOM path, this function can be only used once with one IRQ path,
    /// QEMU will clash if called more than once.
    pub async fn irq_intercept_in(&mut self, qom_path: &str) -> io::Result<Response> {
        let data = format!("irq_intercept_in {}\n", qom_path);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

    /// IRQ intercept out function, intercepts the given IRQ in the given QOM path
    pub async fn irq_intercept_out(&mut self, qom_path: &str) -> io::Result<Response> {
        let data = format!("irq_intercept_out {}\n", qom_path);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

    /// Set IRQ in function, sets the given IRQ in the given QOM path to the given level
    pub async fn set_irq_in(
        &mut self,
        qom_path: &str,
        irq_name: &str,
        line: usize,
        level: isize,
    ) -> io::Result<Response> {
        let data = format!("set_irq_in {} {} {} {}\n", qom_path, irq_name, line, level);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }
}

/// *In & out functions*
macro_rules! impl_in_out {
    ($in:ident, $out:ident, $ty:ty) => {
        impl<T: Socket> Parser<T> {
            pub async fn $in(&mut self, addr: usize) -> io::Result<$ty> {
                let data = format!("{} {:#x}\n", stringify!($in), addr);
                self.socket.send(&data).await?;
                let response = self.response_queue.recv().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Could not receive response")
                })?;

                match response {
                    Response::OkVal(val) => <$ty>::from_str_radix(val.trim_start_matches("0x"), 16)
                        .map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("Could not parse value: {}\n error {}", val, e),
                            )
                        }),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid response")),
                }
            }

            pub async fn $out(&mut self, addr: usize, val: $ty) -> io::Result<Response> {
                let data = format!("{} {:#x} {:#x}\n", stringify!($out), addr, val);
                self.socket.send(&data).await?;
                self.response_queue.recv().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Could not receive response")
                })
            }
        }
    };
}

impl_in_out!(inb, outb, u8);
impl_in_out!(inw, outw, u16);
impl_in_out!(inl, outl, u32);

/// *Write & Read functions*
macro_rules! impl_write_read {
    ($write:ident, $read:ident, $ty:ty) => {
        impl<T: Socket> Parser<T> {
            /// Write a value to the given address, returns a Ok()
            pub async fn $write(&mut self, addr: usize, val: $ty) -> io::Result<Response> {
                let data = format!("{} {:#x} {:#x}", stringify!($write), addr, val);
                self.socket.send(&data).await?;
                self.response_queue.recv().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Could not receive response")
                })
            }

            /// Reads a value from the given address, returns a result with the value
            pub async fn $read(&mut self, addr: usize) -> io::Result<$ty> {
                let data = format!("{} {:#x}\n", stringify!($read), addr);
                self.socket.send(&data).await?;
                let response = self.response_queue.recv().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Could not receive response")
                })?;

                match response {
                    Response::OkVal(val) => <$ty>::from_str_radix(val.trim_start_matches("0x"), 16)
                        .map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("Could not parse value: {}\n error {}", val, e),
                            )
                        }),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid response")),
                }
            }
        }
    };
}

impl_write_read!(writeb, readb, u8);
impl_write_read!(writew, readw, u16);
impl_write_read!(writel, readl, u32);
impl_write_read!(writeq, readq, u64);

/// *Other memory functions*
impl<T: Socket> Parser<T> {
    /// Reads the given number of bytes from the given address, returns a string with the data.
    pub async fn read(&mut self, addr: usize, size: usize) -> io::Result<String> {
        let data = format!("read {:#x} {}\n", addr, size);
        self.socket.send(&data).await?;
        let response =
            self.response_queue.recv().await.ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "Could not receive response")
            })?;

        match response {
            Response::OkVal(val) => Ok(val),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Invalid response")),
        }
    }

    /// Writes the given data to the given address, returns a Ok() if the write was successful
    pub async fn write(
        &mut self,
        addr: usize,
        data: &str,
        data_len: Option<usize>,
    ) -> io::Result<Response> {
        let len = match data_len {
            Some(len) => len,
            None => data.len(),
        };
        let data = format!(
            "write {:#x} {} 0x{}\n",
            addr,
            len,
            data.trim_start_matches("0x")
        );
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

    /// Writes the given base64 data to the given address, returns a Ok() if the write was successful
    pub async fn b64write(&mut self, addr: usize, data: &str) -> io::Result<Response> {
        let enc_data = ENGINE.encode(data);
        let data = format!("b64write {:#x} {} {}\n", addr, data.len(), enc_data);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }
}

/// Used to read data from the qtest socket, should not be used by the user
struct Reader {
    /// Receiver for the socket data
    rx_socket: mpsc::Receiver<String>,
    /// Sender for IRQ data
    tx_irq: mpsc::Sender<Irq>,
    /// Sender for Response data
    tx_response: mpsc::Sender<Response>,
}

impl Reader {
    /// Create a new reader instance with the given receivers and senders
    fn new(
        rx_socket: mpsc::Receiver<String>,
        tx_irq: mpsc::Sender<Irq>,
        tx_response: mpsc::Sender<Response>,
    ) -> Self {
        Self {
            rx_socket,
            tx_irq,
            tx_response,
        }
    }

    /// Reads data from the socket and sends it to the IRQ or Response channels
    async fn read(&mut self) -> io::Result<()> {
        while let Some(raw_data) = self.rx_socket.recv().await {
            let string_data = raw_data.trim_matches(char::from(0)).to_string();

            let lines = string_data.lines();

            for line in lines {
                if line.is_empty() {
                    continue;
                }

                match Irq::try_from(line) {
                    Ok(irq) => self.tx_irq.send(irq).await.map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, format!("Could not send IRQ: {e}"))
                    }),
                    Err(_) => self
                        .tx_response
                        .send(Response::from(string_data.as_str()))
                        .await
                        .map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("Could not send response: {e}"),
                            )
                        }),
                }?;
            }
        }
        Ok(())
    }
}
