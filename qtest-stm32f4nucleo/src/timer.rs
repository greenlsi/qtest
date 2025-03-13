pub mod timer_registers;
use std::ops::Deref;

use qtest::register::Register;
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
