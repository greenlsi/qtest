use std::{io, isize};

use tokio::sync::mpsc;

use crate::parser::irq::IRQ;
use crate::socket::Socket;

pub mod irq;

#[derive(Debug)]
pub struct Parser<T: Socket> {
    socket: T,

    response_queue: mpsc::Receiver<Response>,
}

impl<T: Socket> Parser<T> {
    pub async fn new(url: &str) -> io::Result<(Parser<T>, mpsc::Receiver<IRQ>)> {
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
}

// Clock Management functions
impl<T: Socket> Parser<T> {
    pub async fn clock_step(&mut self, ns: Option<usize>) -> io::Result<Response> {
        let data = match ns {
            Some(ns) => format!("clock_step {}\n", ns),
            None => "clock_step\n".to_string(),
        };
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

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
}

// IRQ Management functions
impl<T: Socket> Parser<T> {
    pub async fn irq_intercept_in(&mut self, qom_path: &str) -> io::Result<Response> {
        let data = format!("irq_intercept_in {}\n", qom_path);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

    pub async fn irq_intercept_out(&mut self, qom_path: &str) -> io::Result<Response> {
        let data = format!("irq_intercept_out {}\n", qom_path);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }

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

// Memory Management functions
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

macro_rules! impl_write_read {
    ($write:ident, $read:ident, $ty:ty) => {
        impl<T: Socket> Parser<T> {
            pub async fn $write(&mut self, addr: usize, val: $ty) -> io::Result<Response> {
                let data = format!("{} {:#x} {:#x}", stringify!($write), addr, val);
                self.socket.send(&data).await?;
                self.response_queue.recv().await.ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "Could not receive response")
                })
            }

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

impl<T: Socket> Parser<T> {
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

    pub async fn b64write(&mut self, addr: usize, data: &str) -> io::Result<Response> {
        let enc_data = base64::encode(data);
        let data = format!("b64write {:#x} {} {}\n", addr, data.len(), enc_data);
        self.socket.send(&data).await?;
        self.response_queue
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Could not receive response"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Response {
    Ok,
    OkVal(String),
    Err(String),
}

impl From<&str> for Response {
    fn from(s: &str) -> Self {
        let mut s_parts = s.split_whitespace();
        if s_parts.next() != Some("OK") {
            return Self::Err(s.to_string());
        }
        match s_parts.next() {
            Some(val) => Self::OkVal(val.to_string()),
            None => Self::Ok,
        }
    }
}

struct Reader {
    rx_socket: mpsc::Receiver<String>,
    tx_irq: mpsc::Sender<IRQ>,
    tx_response: mpsc::Sender<Response>,
}

impl Reader {
    pub fn new(
        rx_socket: mpsc::Receiver<String>,
        tx_irq: mpsc::Sender<IRQ>,
        tx_response: mpsc::Sender<Response>,
    ) -> Self {
        Self {
            rx_socket,
            tx_irq,
            tx_response,
        }
    }

    pub async fn read(&mut self) -> io::Result<()> {
        while let Some(raw_data) = self.rx_socket.recv().await {
            let string_data = raw_data.trim_matches(char::from(0)).to_string();

            let lines = string_data.lines();

            for line in lines {
                if line.is_empty() {
                    continue;
                }

                match IRQ::try_from(line) {
                    Ok(irq) => self.tx_irq.send(irq).await.map_err(|e| {
                        io::Error::new(io::ErrorKind::Other, format!("Could not send IRQ: {}", e))
                    }),
                    Err(_) => self
                        .tx_response
                        .send(Response::from(string_data.as_str()))
                        .await
                        .map_err(|e| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("Could not send response: {}", e),
                            )
                        }),
                }?;
            }
        }
        Ok(())
    }
}
