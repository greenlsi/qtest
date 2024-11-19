/// Parser module, interface to interact with qtest
pub mod parser;
/// Socket module, used to serve and manage qtest socket connections.
pub mod socket;

/// QTest Response enum
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Response {
    /// Successfull response, without any additional data
    Ok,
    /// Successfull response, with additional data
    OkVal(String),
    /// Error in processing the request
    Err(String),
}

// Converts a qtest response string to a Response enum
impl From<&str> for Response {
    fn from(s: &str) -> Self {
        let mut s_parts = s.split_whitespace();
        if s_parts.next() != Some("OK") {
            return Self::Err(s.to_string());
        }
        match s_parts.next() {
            Some(val) => {
                let msg = std::iter::once(val)
                    .chain(s_parts)
                    .collect::<Vec<_>>()
                    .join(" ");
                Self::OkVal(msg)
            }
            None => Self::Ok,
        }
    }
}

/// Struct for defining IRQ events propagated by QEMU.
///
/// The line and state depends on the machine that emits the event.
/// Refer to QEMU documentation for your desired machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Irq {
    /// The line of the IRQ event
    pub line: usize,
    /// The state of the IRQ event
    pub state: IrqState,
}

impl Irq {
    /// Creates a new IRQ instance
    pub fn new(line: usize, state: IrqState) -> Self {
        Irq { line, state }
    }
}

/// Enum for defining the state of an IRQ event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrqState {
    /// The IRQ event is raised
    Raise,
    /// The IRQ event is lowered
    Lower,
}

impl TryFrom<&str> for Irq {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s_parts = s.split_whitespace();

        if s_parts.next() != Some("IRQ") {
            return Err("Invalid IRQ string");
        }
        let ty = match s_parts.next() {
            Some("raise") => IrqState::Raise,
            Some("lower") => IrqState::Lower,
            _ => return Err("Invalid IRQ type"),
        };
        let line = s_parts
            .next()
            .ok_or("Invalid IRQ line")?
            .parse()
            .map_err(|_| "Invalid IRQ line")?;

        match s_parts.next() {
            Some(_) => Err("Invalid IRQ string"),
            None => Ok(Irq::new(line, ty)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_response_from() {
        let response = Response::from("OK");
        assert_eq!(response, Response::Ok);

        let response = Response::from("OK val");
        assert_eq!(response, Response::OkVal("val".to_string()));

        let response = Response::from("ERR error");
        assert_eq!(response, Response::Err("ERR error".to_string()));
    }

    #[test]
    fn test_irq_try_from() {
        let irq = Irq::try_from("invalid");
        assert_eq!(irq, Err("Invalid IRQ string"));

        let irq = Irq::try_from("IRQ invalid");
        assert_eq!(irq, Err("Invalid IRQ type"));

        let irq = Irq::try_from("IRQ raise -1");
        assert_eq!(irq, Err("Invalid IRQ line"));

        let irq = Irq::try_from("IRQ raise 1 invalid");
        assert_eq!(irq, Err("Invalid IRQ string"));

        let irq = Irq::try_from("IRQ raise 1");
        assert_eq!(irq, Ok(Irq::new(1, IrqState::Raise)));

        let irq = Irq::try_from("IRQ lower 2");
        assert_eq!(irq, Ok(Irq::new(2, IrqState::Lower)));
    }
}
