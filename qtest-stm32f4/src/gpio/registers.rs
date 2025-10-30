use qtest::session::Session;
use std::io;

qtest::register!(
    Moder, u32;
    Otyper, u32;
    Bsr, u32;
    Ospeedr, u32;
    Pupdr, u32;
    Idr, u32;
    Odr, u32;
    Bsrr, u32;
    Lckr, u32;
    Afrl, u32;
    Afrh, u32
);

impl Moder {
    /// Returns the mode of a specific pin as a string.
    pub async fn get_mode(&self, pin: usize, session: &mut Session) -> io::Result<String> {
        let value = self.register.read(session).await?;
        let mode = (value >> (2 * pin)) & 0b11;
        let mode = match mode {
            0b00 => "Input",
            0b01 => "Output",
            0b10 => "Alternate Function",
            0b11 => "Analog",
            _ => unreachable!(),
        };
        Ok(mode.to_string())
    }
}

impl Idr {
    /// Returns whether the input value of a pin is high
    pub async fn is_high(&self, pin: usize, session: &mut Session) -> io::Result<bool> {
        let value = self.register.read(session).await?;
        Ok((value & (1 << pin)) != 0)
    }
}

impl Odr {
    /// Returns whether the desired output value of a pin is high
    pub async fn is_high(&self, pin: usize, session: &mut Session) -> io::Result<bool> {
        let value = self.register.read(session).await?;
        Ok((value & (1 << pin)) != 0)
    }
}
