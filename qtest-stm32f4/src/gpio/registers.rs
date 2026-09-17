use super::report::{GpioPinMode, OutputSpeed, OutputType, Pupd};
use qtest::session::Session;
use std::io::{Error, ErrorKind, Result};

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
    pub async fn get_mode(&self, pin: usize, session: &mut Session) -> Result<GpioPinMode> {
        let value = self.register.read(session).await?;
        let mode = (value >> (2 * pin)) & 0b11;
        GpioPinMode::try_from(mode)
    }
}

impl Otyper {
    /// Returns the output type of a specific pin
    pub async fn get_output_type(&self, pin: usize, session: &mut Session) -> Result<OutputType> {
        let value = self.register.read(session).await?;
        let out_type = (value >> pin) & 0b1;
        Ok(OutputType::from(out_type != 0))
    }
}

impl Ospeedr {
    /// Returns the output speed of a specific pin
    pub async fn get_output_speed(&self, pin: usize, session: &mut Session) -> Result<OutputSpeed> {
        let value = self.register.read(session).await?;
        let speed = (value >> (pin * 2)) & 0b11;
        OutputSpeed::try_from(speed)
    }
}

impl Pupdr {
    /// Returns the pull-up/pull-down configuration of a specific pin
    pub async fn get_pupd(&self, pin: usize, session: &mut Session) -> Result<Pupd> {
        let value = self.register.read(session).await?;
        let pupd = (value >> (pin * 2)) & 0b11;
        Pupd::try_from(pupd)
    }
}

impl Idr {
    /// Returns whether the input value of a pin is high
    pub async fn is_high(&self, pin: usize, session: &mut Session) -> Result<bool> {
        let value = self.register.read(session).await?;
        Ok((value & (1 << pin)) != 0)
    }
}

impl Afrl {
    /// Gets the alternate function of a specific pin (0-7)
    pub async fn get_alternate_function(&self, pin: usize, session: &mut Session) -> Result<u8> {
        if pin > 7 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Pin number must be between 0 and 7",
            ));
        }
        let value = self.register.read(session).await?;
        let function = (value >> (pin * 4)) & 0xF;
        Ok(function as u8)
    }
}

impl Afrh {
    /// Gets the alternate function of a specific pin (8-15)
    pub async fn get_alternate_function(&self, pin: usize, session: &mut Session) -> Result<u8> {
        if !(8..=15).contains(&pin) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Pin number must be between 8 and 15",
            ));
        }
        let value = self.register.read(session).await?;
        let function = (value >> ((pin - 8) * 4)) & 0xF;
        Ok(function as u8)
    }
}
