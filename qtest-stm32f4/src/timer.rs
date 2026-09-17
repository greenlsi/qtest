pub mod registers;
pub mod report;

use qtest::{session::Session, utils::Peripheral};
use registers::{
    Arr, Ccer, Ccmr1, Ccmr2, Ccr1, Ccr2, Ccr3, Ccr4, Cnt, Cr1, Cr2, Dcr, Dier, Dmar, Egr, Or, Psc,
    Smcr, Sr,
};
use report::TimerReport;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Timer {
    Tim2,
    Tim5,
}

impl Timer {
    /// Returns the base address of the Timer peripheral
    pub const fn base_addr(&self) -> usize {
        match self {
            Timer::Tim2 => 0x40000000,
            Timer::Tim5 => 0x40000C00,
        }
    }

    /// Reads all Timer registers and returns a parsed [`TimerReport`]
    pub async fn read(&self, session: &mut Session) -> Result<TimerReport> {
        // read 10 subsequent registers of 4 bytes each
        let regs = session.read_u32_le(self.base_addr(), 21).await?;

        Ok(TimerReport {
            cr1: regs[0],
            cr2: regs[1],
            smcr: regs[2],
            dier: regs[3],
            sr: regs[4],
            // skip egr write-only register
            ccmr1: regs[6],
            ccmr2: regs[7],
            ccer: regs[8],
            cnt: regs[9],
            psc: regs[10],
            arr: regs[11],
            // skip reserved word
            ccr1: regs[13],
            ccr2: regs[14],
            ccr3: regs[15],
            ccr4: regs[16],
            // skip reserved word
            dcr: regs[18],
            dmar: regs[19],
            or: regs[20],
        })
    }

    /// Accessor method for the CR1 register
    pub fn cr1(&self) -> Cr1 {
        Cr1::new(self.base_addr())
    }

    /// Accessor method for the CR2 register
    pub fn cr2(&self) -> Cr2 {
        Cr2::new(self.base_addr() + 0x04)
    }

    /// Accessor method for the SMCR register
    pub fn smcr(&self) -> Smcr {
        Smcr::new(self.base_addr() + 0x08)
    }

    /// Accessor method for the DIER register
    pub fn dier(&self) -> Dier {
        Dier::new(self.base_addr() + 0x0C)
    }

    /// Accessor method for the SR register
    pub fn sr(&self) -> Sr {
        Sr::new(self.base_addr() + 0x10)
    }

    /// Accessor method for the EGR register
    pub fn egr(&self) -> Egr {
        Egr::new(self.base_addr() + 0x14)
    }

    /// Accessor method for the CCMR1 register
    pub fn ccmr1(&self) -> Ccmr1 {
        Ccmr1::new(self.base_addr() + 0x18)
    }

    /// Accessor method for the CCMR2 register
    pub fn ccmr2(&self) -> Ccmr2 {
        Ccmr2::new(self.base_addr() + 0x1C)
    }

    /// Accessor method for the CCER register
    pub fn ccer(&self) -> Ccer {
        Ccer::new(self.base_addr() + 0x20)
    }

    /// Accessor method for the CNT register
    pub fn cnt(&self) -> Cnt {
        Cnt::new(self.base_addr() + 0x24)
    }

    /// Accessor method for the PSC register
    pub fn psc(&self) -> Psc {
        Psc::new(self.base_addr() + 0x28)
    }

    /// Accessor method for the ARR register
    pub fn arr(&self) -> Arr {
        Arr::new(self.base_addr() + 0x2C)
    }

    /// Accessor method for the CCR1 register
    pub fn ccr1(&self) -> Ccr1 {
        Ccr1::new(self.base_addr() + 0x34)
    }

    /// Accessor method for the CCR2 register
    pub fn ccr2(&self) -> Ccr2 {
        Ccr2::new(self.base_addr() + 0x38)
    }

    /// Accessor method for the CCR3 register
    pub fn ccr3(&self) -> Ccr3 {
        Ccr3::new(self.base_addr() + 0x3C)
    }

    /// Accessor method for the CCR4 register
    pub fn ccr4(&self) -> Ccr4 {
        Ccr4::new(self.base_addr() + 0x40)
    }

    /// Accessor method for the DCR register
    pub fn dcr(&self) -> Dcr {
        Dcr::new(self.base_addr() + 0x48)
    }

    /// Accessor method for the DMAR register
    pub fn dmar(&self) -> Dmar {
        Dmar::new(self.base_addr() + 0x4C)
    }

    /// Accessor method for the OR register
    pub fn or(&self) -> Or {
        Or::new(self.base_addr() + 0x50)
    }
}

impl Peripheral for Timer {
    fn name(&self) -> &str {
        match self {
            Timer::Tim2 => "tim2",
            Timer::Tim5 => "tim5",
        }
    }

    fn qom_suffix(&self) -> &str {
        match self {
            Timer::Tim2 => "timer[2]",
            Timer::Tim5 => "timer[5]",
        }
    }
    fn out_irq_index(&self) -> Option<usize> {
        None
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Channel {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
}

impl Channel {
    pub fn index(&self) -> usize {
        match self {
            Channel::Ch1 => 1,
            Channel::Ch2 => 2,
            Channel::Ch3 => 3,
            Channel::Ch4 => 4,
        }
    }

    pub fn try_from(index: usize) -> Result<Self> {
        match index {
            1 => Ok(Channel::Ch1),
            2 => Ok(Channel::Ch2),
            3 => Ok(Channel::Ch3),
            4 => Ok(Channel::Ch4),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "Channel index must be between 1 and 4",
            )),
        }
    }
}
