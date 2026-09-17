use crate::{session::Session, Irq, Response};
use tokio::io::{Error, ErrorKind, Result};

pub trait Peripheral: Send {
    fn name(&self) -> &str;

    fn qom_suffix(&self) -> &str;

    fn input_irq_name(&self) -> &str {
        "input-in"
    }

    fn output_irq_name(&self) -> &str {
        "output-out"
    }

    fn out_irq_index(&self) -> Option<usize>;
}

pub trait SoC: Send + Sync {
    fn name(&self) -> &str;

    fn qom_path(&self) -> &str {
        "/machine/soc"
    }

    fn get_peripheral(&self, name: &str) -> Option<Box<dyn Peripheral>>;

    fn input_irq_to_peripheral(&self, irq: Irq) -> Option<Box<dyn Peripheral>>;
}

impl SoC for () {
    fn name(&self) -> &str {
        "soc"
    }

    fn get_peripheral(&self, _name: &str) -> Option<Box<dyn Peripheral>> {
        None
    }

    fn input_irq_to_peripheral(&self, _irq: Irq) -> Option<Box<dyn Peripheral>> {
        None
    }
}

pub async fn set_peripheral_irq<T: SoC>(
    session: &mut Session,
    soc: &T,
    peripheral_name: &str,
    line: usize,
    level: isize,
) -> Result<Response> {
    let peripheral = soc
        .get_peripheral(peripheral_name)
        .ok_or_else(|| Error::new(ErrorKind::NotFound, "Peripheral not found"))?;

    let qom_path = format!("{}/{}", soc.qom_path(), peripheral.qom_suffix());
    session
        .set_irq_in(&qom_path, peripheral.input_irq_name(), line, level)
        .await
}
