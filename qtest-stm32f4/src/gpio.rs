pub mod registers;
pub mod report;

use qtest::{session::Session, utils::Peripheral};
use report::GpioReport;
use std::io::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Gpio {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Gpio {
    /// Returns the base address of the GPIO peripheral
    pub const fn base_addr(&self) -> usize {
        match self {
            Gpio::A => 0x40020000,
            Gpio::B => 0x40020400,
            Gpio::C => 0x40020800,
            Gpio::D => 0x40020C00,
            Gpio::E => 0x40021000,
            Gpio::F => 0x40021400,
            Gpio::G => 0x40021800,
            Gpio::H => 0x40021C00,
        }
    }

    /// Reads all GPIO registers and returns a parsed [`GpioReport`]
    pub async fn read(&self, session: &mut Session) -> Result<GpioReport> {
        let regs = session.read_u32_le(self.base_addr(), 10).await?;

        Ok(GpioReport {
            moder: regs[0],
            otyper: regs[1],
            ospeedr: regs[2],
            pupdr: regs[3],
            idr: regs[4],
            // ignore odr and bsrr write-only registers
            lckr: regs[7],
            afrl: regs[8],
            afrh: regs[9],
        })
    }

    /// Accessor method for the MODER register
    pub fn moder(&self) -> registers::Moder {
        registers::Moder::new(self.base_addr())
    }

    /// Accessor method for the OTYPER register
    pub fn otyper(&self) -> registers::Otyper {
        registers::Otyper::new(self.base_addr() + 0x04)
    }

    /// Accessor method for the OSPEEDR register
    pub fn ospeedr(&self) -> registers::Ospeedr {
        registers::Ospeedr::new(self.base_addr() + 0x08)
    }

    /// Accessor method for the PUPDR register
    pub fn pupdr(&self) -> registers::Pupdr {
        registers::Pupdr::new(self.base_addr() + 0x0C)
    }

    /// Accessor method for the IDR register
    pub fn idr(&self) -> registers::Idr {
        registers::Idr::new(self.base_addr() + 0x10)
    }

    /// Accessor method for the ODR register
    pub fn odr(&self) -> registers::Odr {
        registers::Odr::new(self.base_addr() + 0x14)
    }

    /// Accessor method for the BSRR register
    pub fn bsrr(&self) -> registers::Bsrr {
        registers::Bsrr::new(self.base_addr() + 0x18)
    }

    /// Accessor method for the LCKR register
    pub fn lckr(&self) -> registers::Lckr {
        registers::Lckr::new(self.base_addr() + 0x1C)
    }

    /// Accessor method for the AFRL register
    pub fn afrl(&self) -> registers::Afrl {
        registers::Afrl::new(self.base_addr() + 0x20)
    }

    /// Accessor method for the AFRH register
    pub fn afrh(&self) -> registers::Afrh {
        registers::Afrh::new(self.base_addr() + 0x24)
    }
}

impl Peripheral for Gpio {
    fn name(&self) -> &str {
        match self {
            Gpio::A => "gpioa",
            Gpio::B => "gpiob",
            Gpio::C => "gpioc",
            Gpio::D => "gpiod",
            Gpio::E => "gpioe",
            Gpio::F => "gpiof",
            Gpio::G => "gpiog",
            Gpio::H => "gpioh",
        }
    }
    fn qom_suffix(&self) -> &str {
        match self {
            Gpio::A => "gpio[0]",
            Gpio::B => "gpio[1]",
            Gpio::C => "gpio[2]",
            Gpio::D => "gpio[3]",
            Gpio::E => "gpio[4]",
            Gpio::F => "gpio[5]",
            Gpio::G => "gpio[6]",
            Gpio::H => "gpio[7]",
        }
    }

    fn out_irq_index(&self) -> Option<usize> {
        match self {
            Gpio::A => Some(0),
            Gpio::B => Some(1),
            Gpio::C => Some(2),
            Gpio::D => Some(3),
            Gpio::E => Some(4),
            Gpio::F => Some(5),
            Gpio::G => Some(6),
            Gpio::H => Some(7),
        }
    }
}
