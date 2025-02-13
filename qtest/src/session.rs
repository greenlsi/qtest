use crate::{
    dispatcher::Dispatcher,
    socket::{SocketReader, SocketWriter},
    Irq, Response,
};
use base64::{
    alphabet,
    engine::{Engine, GeneralPurpose, GeneralPurposeConfig},
};
use tokio::{
    io::{Error, ErrorKind, Result},
    sync::mpsc::{channel, Receiver},
    task::JoinHandle,
};

const ENGINE: GeneralPurpose =
    GeneralPurpose::new(&alphabet::STANDARD, GeneralPurposeConfig::new());

pub struct Session {
    socket_writer: SocketWriter,
    dispatcher_handle: JoinHandle<Result<()>>,
    response_receiver: Receiver<Response>,
    irq_receiver: Option<Receiver<Irq>>,
}

impl Session {
    pub(crate) fn new(socket_reader: SocketReader, socket_writer: SocketWriter) -> Self {
        let (response_sender, response_receiver) = channel(32);
        let (irq_sender, irq_receiver) = channel(32);
        let dispatcher = Dispatcher::new(socket_reader, response_sender, irq_sender);

        Self {
            socket_writer,
            dispatcher_handle: dispatcher.spawn(),
            response_receiver,
            irq_receiver: Some(irq_receiver),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.dispatcher_handle.is_finished()
    }

    pub fn abort(self) {
        self.dispatcher_handle.abort()
    }

    async fn get_response(&mut self) -> Result<Response> {
        self.response_receiver
            .recv()
            .await
            .ok_or_else(|| Error::new(ErrorKind::Other, "Could not receive response"))
    }

    /// Clock step function, steps the clock by the given number of nanoseconds
    pub async fn clock_step(&mut self, ns: Option<usize>) -> Result<Response> {
        let data = match ns {
            Some(ns) => format!("clock_step {ns}\n"),
            None => "clock_step\n".to_string(),
        };
        self.socket_writer.write(&data).await?;
        self.get_response().await
    }

    /// Set the clock to the given number of nanoseconds
    pub async fn clock_set(&mut self, ns: usize) -> Result<usize> {
        let data = format!("clock_set {ns}\n");
        self.socket_writer.write(&data).await?;

        match self.get_response().await? {
            Response::OkVal(val) => val.parse().map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Could not parse value: {val} ({e})"),
                )
            }),
            Response::Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("invalid response: {e}"),
            )),
            _ => Err(Error::new(ErrorKind::Other, "Invalid response")),
        }
    }

    /// Set IRQ in function, sets the given IRQ in the given QOM path to the given level
    pub async fn set_irq_in(
        &mut self,
        qom_path: &str,
        irq_name: &str,
        line: usize,
        level: isize,
    ) -> Result<Response> {
        let data = format!("set_irq_in {qom_path} {irq_name} {line} {level}\n");
        self.socket_writer.write(&data).await?;
        self.get_response().await
    }

    /// IRQ intercept in function, intercepts the given IRQ in the given QOM path.
    ///
    /// # Note
    ///
    /// In QEMU, it is only possible to intercept one interrupt source.
    /// The second call to this function will return an error.
    pub async fn irq_intercept_in(&mut self, qom_path: &str) -> Result<Receiver<Irq>> {
        self.irq_intercept(qom_path, "in").await
    }

    /// IRQ intercept out function, intercepts the given IRQ in the given QOM path.
    ///
    /// # Note
    ///
    /// In QEMU, it is only possible to intercept one interrupt source.
    /// The second call to this function will return an error.
    pub async fn irq_intercept_out(&mut self, qom_path: &str) -> Result<Receiver<Irq>> {
        self.irq_intercept(qom_path, "out").await
    }

    async fn irq_intercept(&mut self, qom_path: &str, intercept: &str) -> Result<Receiver<Irq>> {
        match self.irq_receiver.is_some() {
            true => {
                let data = format!("irq_intercept_{intercept} {qom_path}\n");
                self.socket_writer.write(&data).await?;
                match self.get_response().await? {
                    Response::Ok => Ok(self.irq_receiver.take().unwrap()),
                    other => Err(Error::new(
                        ErrorKind::Other,
                        format!("Invalid response: {other:?}"),
                    )),
                }
            }
            false => Err(Error::new(ErrorKind::Other, "IRQ receiver already taken")),
        }
    }
}

/// *In & out functions*
macro_rules! impl_in_out {
    ($in:ident, $out:ident, $ty:ty) => {
        impl Session {
            pub async fn $in(&mut self, addr: usize) -> Result<$ty> {
                let data = format!("{} {:#x}\n", stringify!($in), addr);
                self.socket_writer.write(&data).await?;

                match self.get_response().await? {
                    Response::OkVal(val) => <$ty>::from_str_radix(val.trim_start_matches("0x"), 16)
                        .map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Could not parse value: {} ({})", val, e),
                            )
                        }),
                    _ => Err(Error::new(ErrorKind::Other, "Invalid response")),
                }
            }

            pub async fn $out(&mut self, addr: usize, val: $ty) -> Result<Response> {
                let data = format!("{} {:#x} {:#x}\n", stringify!($out), addr, val);
                self.socket_writer.write(&data).await?;
                self.get_response().await
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
        impl Session {
            /// Write a value to the given address, returns a Ok()
            pub async fn $write(&mut self, addr: usize, val: $ty) -> Result<Response> {
                let data = format!("{} {:#x} {:#x}", stringify!($write), addr, val);
                self.socket_writer.write(&data).await?;
                self.get_response().await
            }

            /// Reads a value from the given address, returns a result with the value
            pub async fn $read(&mut self, addr: usize) -> Result<$ty> {
                let data = format!("{} {:#x}\n", stringify!($read), addr);
                self.socket_writer.write(&data).await?;

                match self.get_response().await? {
                    Response::OkVal(val) => <$ty>::from_str_radix(val.trim_start_matches("0x"), 16)
                        .map_err(|e| {
                            Error::new(
                                ErrorKind::Other,
                                format!("Could not parse value: {}\n error {}", val, e),
                            )
                        }),
                    _ => Err(Error::new(ErrorKind::Other, "Invalid response")),
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
impl Session {
    /// Reads the given number of bytes from the given address, returns a string with the data.
    pub async fn read(&mut self, addr: usize, size: usize) -> Result<String> {
        let data = format!("read {addr:#x} {size}\n");
        self.socket_writer.write(&data).await?;
        match self.get_response().await? {
            Response::OkVal(val) => Ok(val),
            _ => Err(Error::new(ErrorKind::Other, "Invalid response")),
        }
    }

    /// Writes the given data to the given address, returns a Ok() if the write was successful
    pub async fn write(
        &mut self,
        addr: usize,
        data: &str,
        data_len: Option<usize>,
    ) -> Result<Response> {
        let len = match data_len {
            Some(len) => len,
            None => data.len(),
        };
        let data = format!(
            "write {addr:#x} {len} 0x{}\n",
            data.trim_start_matches("0x")
        );
        self.socket_writer.write(&data).await?;
        self.get_response().await
    }

    /// Writes the given base64 data to the given address, returns a Ok() if the write was successful
    pub async fn b64write(&mut self, addr: usize, data: &str) -> Result<Response> {
        let enc_data = ENGINE.encode(data);
        let data = format!("b64write {addr:#x} {} {enc_data}\n", data.len());
        self.socket_writer.write(&data).await?;
        self.get_response().await
    }
}
