use std::io::{Error, ErrorKind, Result};

/// Complete report of a GPIO peripheral. It contains the values of all relevant registers.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GpioReport {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
}

impl GpioReport {
    pub fn pin_report(&self, pin: usize) -> Result<GpioPinReport> {
        if pin > 15 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Pin number must be between 0 and 15",
            ));
        }
        let mode = (self.moder >> (pin * 2)) & 0b11;
        let out_type = ((self.otyper >> pin) & 0b1) != 0;
        let out_speed = (self.ospeedr >> (pin * 2)) & 0b11;
        let pupd = (self.pupdr >> (pin * 2)) & 0b11;
        let data = ((self.idr >> pin) & 0b1) != 0;
        let locked = ((self.lckr >> pin) & 0b1) != 0;
        let alt_function = if pin < 8 {
            (self.afrl >> (pin * 4)) & 0b1111
        } else {
            (self.afrh >> ((pin - 8) * 4)) & 0b1111
        };

        Ok(GpioPinReport::new(
            GpioPinMode::try_from(mode)?,
            out_type.into(),
            OutputSpeed::try_from(out_speed)?,
            Pupd::try_from(pupd)?,
            data,
            alt_function as u8,
            locked,
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "mode"))]
pub enum GpioPinReport {
    Input {
        pupd: Pupd,
        input: GpioPinState,
        locked: bool,
    },
    Output {
        out_type: OutputType,
        pupd: Pupd,
        speed: OutputSpeed,
        output: GpioPinState,
        locked: bool,
    },
    Alternate {
        out_type: OutputType,
        pupd: Pupd,
        speed: OutputSpeed,
        alt_function: u8,
        locked: bool,
    },
    Analog {
        locked: bool,
    },
}

impl GpioPinReport {
    pub fn new(
        mode: GpioPinMode,
        output_type: OutputType,
        output_speed: OutputSpeed,
        pupd: Pupd,
        data: bool,
        alt_function: u8,
        locked: bool,
    ) -> Self {
        match mode {
            GpioPinMode::Input => GpioPinReport::Input {
                pupd,
                input: data.into(),
                locked,
            },
            GpioPinMode::Output => GpioPinReport::Output {
                out_type: output_type,
                pupd,
                speed: output_speed,
                output: data.into(),
                locked,
            },
            GpioPinMode::Alternate => GpioPinReport::Alternate {
                out_type: output_type,
                pupd,
                speed: output_speed,
                alt_function,
                locked,
            },
            GpioPinMode::Analog => GpioPinReport::Analog { locked },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GpioPinMode {
    Input,
    Output,
    Alternate,
    Analog,
}

impl GpioPinMode {
    pub fn try_from(value: u32) -> Result<Self> {
        match value {
            0b00 => Ok(GpioPinMode::Input),
            0b01 => Ok(GpioPinMode::Output),
            0b10 => Ok(GpioPinMode::Alternate),
            0b11 => Ok(GpioPinMode::Analog),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid mode value")),
        }
    }
}

/// Output type configuration (for output and alternate function modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputType {
    PushPull,
    OpenDrain,
}

impl From<bool> for OutputType {
    fn from(value: bool) -> Self {
        match value {
            false => OutputType::PushPull,
            true => OutputType::OpenDrain,
        }
    }
}

/// Output speed configuration (for output and alternate function modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OutputSpeed {
    Low,
    Medium,
    Fast,
    High,
}

impl OutputSpeed {
    pub fn try_from(value: u32) -> Result<Self> {
        match value {
            0b00 => Ok(OutputSpeed::Low),
            0b01 => Ok(OutputSpeed::Medium),
            0b10 => Ok(OutputSpeed::Fast),
            0b11 => Ok(OutputSpeed::High),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid output speed value",
            )),
        }
    }
}

/// Pull-Up/Pull-Down configuration (for input, output, and alternate function modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Pupd {
    None,
    PullUp,
    PullDown,
}

impl From<Pupd> for u32 {
    fn from(value: Pupd) -> Self {
        match value {
            Pupd::None => 0b00,
            Pupd::PullUp => 0b01,
            Pupd::PullDown => 0b10,
        }
    }
}

impl Pupd {
    pub fn try_from(value: u32) -> Result<Self> {
        match value {
            0b00 => Ok(Pupd::None),
            0b01 => Ok(Pupd::PullUp),
            0b10 => Ok(Pupd::PullDown),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid pull-up/pull-down value",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GpioPinState {
    Low,
    High,
}

impl From<bool> for GpioPinState {
    fn from(value: bool) -> Self {
        match value {
            false => GpioPinState::Low,
            true => GpioPinState::High,
        }
    }
}

impl From<GpioPinState> for bool {
    fn from(state: GpioPinState) -> Self {
        match state {
            GpioPinState::Low => false,
            GpioPinState::High => true,
        }
    }
}
