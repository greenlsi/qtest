use qtest::parser::Parser;
use qtest::register::Register;
use qtest::socket::Socket;
use std::io;

// Trait común que define operaciones básicas para registros
pub trait RegisterOps {
    fn get_address(&self) -> usize;
    fn get_name(&self) -> &str;
}

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
impl RegisterOps for Moder {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Otyper {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Bsr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Ospeedr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Pupdr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
    pub async fn is_high(&self, pin: usize, parser: &mut Parser<impl Socket>) -> io::Result<bool> {
        // Lee el valor del registro
        match self.register.read_register(parser).await {
            Ok(value) => Ok((value & (1 << pin)) != 0), // Devuelve true si el pin está en alto
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading register: {}", e),
            )),
        }
    }
}
impl RegisterOps for Idr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
    pub async fn is_high(&self, pin: usize, parser: &mut Parser<impl Socket>) -> io::Result<bool> {
        // Lee el valor del registro
        match self.register.read_register(parser).await {
            Ok(value) => Ok((value & (1 << pin)) != 0), // Devuelve true si el pin está en alto
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading register: {}", e),
            )),
        }
    }
}
impl RegisterOps for Odr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Bsrr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Lckr {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Afrl {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
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
impl RegisterOps for Afrh {
    fn get_address(&self) -> usize {
        self.register.get_address()
    }

    fn get_name(&self) -> &str {
        self.register.get_name()
    }
}
