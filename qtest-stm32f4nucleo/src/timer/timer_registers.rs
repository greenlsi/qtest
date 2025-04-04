use qtest::parser::Parser;
use qtest::register::Register;
use qtest::socket::Socket;
use std::io;


pub trait RegisterOps {
    fn get_address(&self) -> usize;
    fn get_name(&self) -> &str;
}

qtest::register!(Cr1, u16);

qtest::register!(Cr2, u16);

qtest::register!(Smcr, u16);

qtest::register!(Dier, u16);

qtest::register!(Sr, u16);

qtest::register!(Egr, u16);

qtest::register!(Ccmr1, u16);
impl Ccmr1{
    
    
}

qtest::register!(Ccmr2, u16);

qtest::register!(Ccer, u16);

qtest::register!(Cnt, u16);

qtest::register!(Psc, u16);
impl Psc {
    pub async fn get_prescaler(&self, parser: &mut Parser<impl Socket>) ->  io::Result<u16> {
        match self.register.read_register(parser).await {
            Ok(value) => Ok(value as u16),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading PSC register: {}", e),
            )),
        }
        
    }
    
}

qtest::register!(Arr, u16);
impl Arr {
    pub async fn get_auto_reload(&self, parser: &mut Parser<impl Socket>) ->  io::Result<u16> {
        match self.register.read_register(parser).await {
            Ok(value) => Ok(value as u16),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading ARR register: {}", e),
            )),
        }
        
    }
}

qtest::register!(Ccr1, u32);
impl Ccr1 {
    pub async fn get_ccr1(&self, parser: &mut Parser<impl Socket>) ->  io::Result<u32> {
        match self.register.read_register(parser).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR1 register: {}", e),
            )),
        }
        
    }
    
}

qtest::register!(Ccr2, u32);
impl Ccr2{
    pub async fn get_ccr2(&self, parser: &mut Parser<impl Socket>) ->  io::Result<u32> {
        match self.register.read_register(parser).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR2 register: {}", e),
            )),
        }
        
    }
}

qtest::register!(Ccr3, u32);
impl Ccr3{
    pub async fn get_ccr3(&self, parser: &mut Parser<impl Socket>) ->  io::Result<u32> {
        match self.register.read_register(parser).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR3 register: {}", e),
            )),
        }
        
    }
}

qtest::register!(Ccr4, u32);
impl Ccr4{
    pub async fn get_ccr4(&self, parser: &mut Parser<impl Socket>) ->  io::Result<u32> {
        match self.register.read_register(parser).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR4 register: {}", e),
            )),
        }
        
    }
}

qtest::register!(Dcr, u16);

qtest::register!(Dmar, u16);

//ESTE REGISTRO SOLO LO TIENE EL TIMER 2 Y EL 5
qtest::register!(Or, u16);