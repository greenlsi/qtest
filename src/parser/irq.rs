/// Struct for defining out irq events propagated by QEMU, all the items can be accessed directly
///
/// The line and state depends on the machine that emits the event, refer to QEMU documentation for your desired machine
///
/// # Example
///
/// In this example we create a new IRQ instance with line 1 and state Raise
/// ```
/// let irq = IRQ::new(1, IRQState::Raise);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IRQ {
    pub line: u32,

    pub state: IRQState,
}

impl IRQ {
    /// Create a new IRQ instance
    ///
    /// # Example
    /// ```
    /// let irq = IRQ::new(1, IRQState::Raise);
    /// ```
    pub fn new(line: u32, state: IRQState) -> Self {
        IRQ { line, state }
    }
}

/// TryFrom implementation for IRQ, returns a Result with the IRQ instance or an error message if it cannot be parsed successfully
///
/// It tries to parse the input string from the qtest nomeclature, for example "IRQ raise 1" to an IRQ instance, else returns an
/// Err(str) with the input string
///
/// # Example
///
/// In this example we parse a string "IRQ raise 1" to an IRQ instance
/// ```
///  let string = "IRQ raise 1";
///  let irq = IRQ::try_from(string).unwrap();
/// ```
impl TryFrom<&str> for IRQ {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s_parts = s.split_whitespace();

        if s_parts.next() != Some("IRQ") {
            return Err("Invalid IRQ string");
        }
        let ty = match s_parts.next() {
            Some("raise") => IRQState::Raise,
            Some("lower") => IRQState::Lower,
            _ => return Err("Invalid IRQ type"),
        };
        let line = s_parts
            .next()
            .ok_or("Invalid IRQ line")?
            .parse()
            .map_err(|_| "Invalid IRQ line")?;
        Ok(IRQ::new(line, ty))
    }
}

/// Enum for defining the state of an IRQ event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IRQState {
    Raise,
    Lower,
}
