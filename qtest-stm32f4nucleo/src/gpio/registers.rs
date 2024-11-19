use qtest::parser::Parser;
use qtest::register::Register;
use qtest::socket::Socket;

#[derive(Debug, Clone)]
pub struct Moder {
    register: Register<u32>,
}
impl Moder {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("MODER", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Otyper {
    register: Register<u32>,
}

impl Otyper {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("OTYPER", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bsr {
    register: Register<u32>,
}

impl Bsr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("BSR", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ospeedr {
    register: Register<u32>,
}

impl Ospeedr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("OSPEEDR", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pupdr {
    register: Register<u32>,
}

impl Pupdr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("PUPDR", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Idr {
    register: Register<u32>,
}

impl Idr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("IDR", address),
        }
    }
    pub async fn is_high(&self, pin: usize, parser: &mut Parser<impl Socket>) -> bool {
        let value = self.register.read_register(parser).await;
        (value & (1 << pin)) != 0
    }
}

#[derive(Debug, Clone)]
pub struct Odr {
    register: Register<u32>,
}

impl Odr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("ODR", address),
        }
    }
    pub async fn is_high(&self, pin: usize, parser: &mut Parser<impl Socket>) -> bool {
        let value = self.register.read_register(parser).await;
        (value & (1 << pin)) != 0
    }
}

#[derive(Debug, Clone)]
pub struct Bsrr {
    register: Register<u32>,
}

impl Bsrr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("BSRR", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lckr {
    register: Register<u32>,
}

impl Lckr {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("LCKR", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Afrl {
    register: Register<u32>,
}

impl Afrl {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("AFRL", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Afrh {
    register: Register<u32>,
}

impl Afrh {
    pub fn new(address: usize) -> Self {
        Self {
            register: Register::new("AFRH", address),
        }
    }
}
