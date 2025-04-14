pub mod timer_registers;
use std::{ops::Deref, usize};
use std::io;

use qtest::{register::Register, parser::Parser, socket::Socket};
use timer_registers::{Arr, Ccmr1, Ccmr2, Ccr1, Ccr2, Ccr3, Ccr4, Cnt, Cr1, Cr2, Dcr, Dier, Dmar, Egr, Psc, Smcr, Sr, Ccer, Or, RegisterOps};




#[derive(Debug, Clone)]

//estructura para TIM2 y TIM5
pub struct Timer{
    cr1: Cr1,
    cr2: Cr2,
    smcr: Smcr,
    dier: Dier,
    sr: Sr,
    egr: Egr,
    ccmr1: Ccmr1,
    ccmr2: Ccmr2,
    ccer: Ccer,
    cnt: Cnt,
    psc: Psc,
    arr: Arr,
    ccr1: Ccr1,
    ccr2: Ccr2,
    ccr3: Ccr3,
    ccr4: Ccr4,
    dcr: Dcr,
    dmar: Dmar,
    or: Or,
}

macro_rules! create_register_accessors {
    ($($name:ident, $reg:ident, $type:ty),*) => {
        $(
            pub fn $reg(&self) -> &$type {
                &self.$reg
            }
            pub fn $name(&mut self) -> &mut $type {
                &mut self.$reg
            }
        )*
    };
}

impl Timer {
    pub fn new(address: usize) -> Self {
        Timer {
            cr1: Cr1::new(address),
            cr2: Cr2::new(address + 0x04),
            smcr: Smcr::new(address + 0x08),
            dier: Dier::new(address + 0x0C),
            sr: Sr::new(address + 0x10),
            egr: Egr::new(address + 0x14),
            ccmr1: Ccmr1::new(address + 0x18),
            ccmr2: Ccmr2::new(address + 0x1C),
            ccer: Ccer::new(address + 0x20),
            cnt: Cnt::new(address + 0x24),
            psc: Psc::new(address + 0x28),
            arr: Arr::new(address + 0x2C),
            ccr1: Ccr1::new(address + 0x34),
            ccr2: Ccr2::new(address + 0x38),
            ccr3: Ccr3::new(address + 0x3C),
            ccr4: Ccr4::new(address + 0x40),
            dcr: Dcr::new(address + 0x48),
            dmar: Dmar::new(address + 0x4C),
            or: Or::new(address + 0x50),          
        }
    }

    // get_from_name que devuelve el tipo específico
    pub fn get(&self, name: &str) -> Option<&dyn RegisterOps> {
        match name {
            "CR1" => Some(&self.cr1),
            "CR2" => Some(&self.cr2),
            "SMCR" => Some(&self.smcr),
            "DIER" => Some(&self.dier),
            "SR" => Some(&self.sr), 
            "EGR" => Some(&self.egr),
            "CCMR1" => Some(&self.ccmr1),
            "CCMR2" => Some(&self.ccmr2),
            "CCER" => Some(&self.ccer),
            "CNT" => Some(&self.cnt),
            "PSC" => Some(&self.psc),
            "ARR" => Some(&self.arr),
            "CCR1" => Some(&self.ccr1),
            "CCR2" => Some(&self.ccr2),
            "CCR3" => Some(&self.ccr3),
            "CCR4" => Some(&self.ccr4),
            "DCR" => Some(&self.dcr),
            "DMAR" => Some(&self.dmar),
            "OR" => Some(&self.or),
            _ => None,
        }
    }

    pub async fn get_duty_cycle(&self, parser: &mut Parser<impl Socket>,channel: usize) ->  io::Result<u8> {
        let ccr_value = match channel {
            1 => self.ccr1.get_ccr1(parser).await,
            2 => self.ccr2.get_ccr2(parser).await,
            3 => self.ccr3.get_ccr3(parser).await,
            4 => self.ccr4.get_ccr4(parser).await,
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Invalid channel")),
        }?;
    
        let arr_value = self.arr.read_register(parser).await?;
    
        if arr_value == 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "ARR value is zero"));
        }
    
        let duty = ((ccr_value as f32 / arr_value as f32) * 100.0).round() as u8;
        Ok(duty)
    }

    pub fn calculate_pwm_frequency(psc: u16, arr: u16) -> u32 {
        
        let timer_clock_hz = 16_000_000; //COMPROBAR QUE ESTE ES EL VALOR CORRECTO DEL TIMER
        if arr == 0 {
            return 0;
        }
        timer_clock_hz / ((psc as u32 + 1) * (arr as u32 + 1))
    }

    pub async fn full_channel_diagnosis(&self, parser: &mut Parser<impl Socket>) -> io::Result<Vec<ChannelDiagnosis>> {
        let mut report = Vec::new();

        let ccmr1 = self.ccmr1.read_register(parser).await?;
        let ccmr2 = self.ccmr2.read_register(parser).await?;
        let ccer = self.ccer.read_register(parser).await?;
        let psc = self.psc.read_register(parser).await?;
        let arr = self.arr.read_register(parser).await?;

        for channel in 1..=4 {
            let (ccmr_value, offset) = match channel {
                1 => (ccmr1, 0),
                2 => (ccmr1, 8),
                3 => (ccmr2, 0),
                4 => (ccmr2, 8),
                _ => unreachable!(),
            };

            let mode_bits = (ccmr_value >> (4 + offset)) & 0b111;
            let capture_compare_selection = (ccmr_value >> offset) & 0b11;

            let (enable_bit, polarity_bit) = match channel {
                1 => (0, 1),
                2 => (4, 5),
                3 => (8, 9),
                4 => (12, 13),
                _ => (0, 0),
            };

            let enabled = (ccer & (1 << enable_bit)) != 0;
            let polarity = if (ccer & (1 << polarity_bit)) != 0 { "Low" } else { "High" };

            let mode = match capture_compare_selection {
                0b00 => { // Output mode
                    match mode_bits {
                        0b000 => "Frozen (inactive)",
                        0b001 => "Active on match",
                        0b010 => "Inactive on match",
                        0b011 => "Toggle output",
                        0b100 => "Force inactive level",
                        0b101 => "Force active level",
                        0b110 => "PWM mode 1",
                        0b111 => "PWM mode 2",
                        _ => "Unknown output mode",
                    }
                },
                0b01 => {
                    if enabled {
                        "Input capture on TI1 (enabled)"
                    } else {
                        "Input capture on TI1 (disabled)"
                    }
                },
                0b10 => {
                    if enabled {
                        "Input capture on TI2 (enabled)"
                    } else {
                        "Input capture on TI2 (disabled)"
                    }
                },
                0b11 => {
                    if enabled {
                        "Input capture on TRC (enabled)"
                    } else {
                        "Input capture on TRC (disabled)"
                    }
                },
                _ => "Unknown",
            }.to_string();

            let duty_cycle = if enabled && mode.contains("PWM") {
                match channel {
                    1 => self.ccr1.get_ccr1(parser).await.ok(),
                    2 => self.ccr2.get_ccr2(parser).await.ok(),
                    3 => self.ccr3.get_ccr3(parser).await.ok(),
                    4 => self.ccr4.get_ccr4(parser).await.ok(),
                    _ => None,
                }.and_then(|ccr| {
                    if arr == 0 { None } else {
                        Some(((ccr as f32 / arr as f32) * 100.0).round() as u8)
                    }
                })
            } else {
                None
            };

            let frequency = if mode.contains("PWM") && arr != 0 {
                Some(Timer::calculate_pwm_frequency(psc, arr))
            } else {
                None
            };

            report.push(ChannelDiagnosis {
                channel,
                enabled,
                mode,
                polarity: polarity.to_string(),
                duty_cycle,
                frequency,
            });
        }

        Ok(report)
    }
    
        

    create_register_accessors!(
        cr1_mut, cr1, Cr1,
        cr2_mut, cr2, Cr2,
        smcr_mut, smcr, Smcr,
        dier_mut, dier, Dier,
        sr_mut, sr, Sr,
        egr_mut, egr, Egr,
        ccmr1_mut, ccmr1, Ccmr1,
        ccmr2_mut, ccmr2, Ccmr2,
        ccer_mut, ccer, Ccer,
        cnt_mut, cnt, Cnt,
        psc_mut, psc, Psc,
        arr_mut, arr, Arr,
        ccr1_mut, ccr1, Ccr1,
        ccr2_mut, ccr2, Ccr2,
        ccr3_mut, ccr3, Ccr3,
        ccr4_mut, ccr4, Ccr4,
        dcr_mut, dcr, Dcr,
        dmar_mut, dmar, Dmar,
        or_mut, or, Or
    );

}
    pub struct ChannelDiagnosis {
        pub channel: usize,
        pub enabled: bool,
        pub mode: String,
        pub polarity: String,
        pub duty_cycle: Option<u8>, // 0-100 DutyCycle en %
        pub frequency: Option<u32>, // Hz
    }