use qtest::session::Session;
use std::io;

qtest::register!(Cr1, u32);

qtest::register!(Cr2, u32);

qtest::register!(Smcr, u32);

qtest::register!(Dier, u32);

qtest::register!(Sr, u32);

qtest::register!(Egr, u32);

qtest::register!(Ccmr1, u32);

qtest::register!(Ccmr2, u32);

qtest::register!(Ccer, u32);

qtest::register!(Cnt, u32);

qtest::register!(Psc, u32);

impl Psc {
    pub async fn get_prescaler(&self, session: &mut Session) -> io::Result<u32> {
        match self.register.read(session).await {
            Ok(value) => Ok(value),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading PSC register: {}", e),
            )),
        }
    }
}

qtest::register!(Arr, u32);
impl Arr {
    pub async fn get_auto_reload(&self, session: &mut Session) -> io::Result<u32> {
        match self.register.read(session).await {
            Ok(value) => Ok(value),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading ARR register: {}", e),
            )),
        }
    }
}

qtest::register!(Ccr1, u32);
impl Ccr1 {
    pub async fn get_ccr1(&self, session: &mut Session) -> io::Result<u32> {
        match self.register.read(session).await {
            Ok(value) => Ok(value),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR1 register: {}", e),
            )),
        }
    }
}

qtest::register!(Ccr2, u32);
impl Ccr2 {
    pub async fn get_ccr2(&self, session: &mut Session) -> io::Result<u32> {
        match self.register.read(session).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR2 register: {}", e),
            )),
        }
    }
}

qtest::register!(Ccr3, u32);
impl Ccr3 {
    pub async fn get_ccr3(&self, session: &mut Session) -> io::Result<u32> {
        match self.register.read(session).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR3 register: {}", e),
            )),
        }
    }
}

qtest::register!(Ccr4, u32);
impl Ccr4 {
    pub async fn get_ccr4(&self, session: &mut Session) -> io::Result<u32> {
        match self.register.read(session).await {
            Ok(value) => Ok(value as u32),
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Error reading CCR4 register: {}", e),
            )),
        }
    }
}

qtest::register!(Dcr, u32);

qtest::register!(Dmar, u32);

//ESTE REGISTRO SOLO LO TIENE EL TIMER 2 Y EL 5
qtest::register!(Or, u32);
