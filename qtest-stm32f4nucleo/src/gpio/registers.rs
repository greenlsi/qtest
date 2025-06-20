use qtest::session::Session;
use std::io;

qtest::register!(Moder, u32);

impl Moder {
    /// Devuelve el modo de un pin específico en string
    pub async fn get_mode(&self, pin: usize, session: &mut Session) -> io::Result<String> {
        // Lee el valor del registro
        match self.register.read(session).await {
            Ok(value) => {
                // Obtiene el modo del pin
                let mode = (value >> (2 * pin)) & 0b11;
                // Devuelve el modo en string
                match mode {
                    0b00 => Ok("Input".to_string()),
                    0b01 => Ok("Output".to_string()),
                    0b10 => Ok("Alternate Function".to_string()),
                    0b11 => Ok("Analog".to_string()),
                    _ => Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Invalid mode value".to_string(),
                    )),
                }
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading register: {}", e),
            )),
        }
    }
}

qtest::register!(Otyper, u32);

qtest::register!(Bsr, u32);

qtest::register!(Ospeedr, u32);

qtest::register!(Pupdr, u32);

qtest::register!(Idr, u32);

impl Idr {
    pub async fn is_high(&self, pin: usize, session: &mut Session) -> io::Result<bool> {
        // Lee el valor del registro
        match self.register.read(session).await {
            Ok(value) => Ok((value & (1 << pin)) != 0), // Devuelve true si el pin está en alto
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading register: {}", e),
            )),
        }
    }
}

qtest::register!(Odr, u32);

impl Odr {
    pub async fn is_high(&self, pin: usize, session: &mut Session) -> io::Result<bool> {
        // Lee el valor del registro
        match self.register.read(session).await {
            Ok(value) => Ok((value & (1 << pin)) != 0), // Devuelve true si el pin está en alto
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading register: {}", e),
            )),
        }
    }
}

qtest::register!(Bsrr, u32);

qtest::register!(Lckr, u32);

qtest::register!(Afrl, u32);

qtest::register!(Afrh, u32);
