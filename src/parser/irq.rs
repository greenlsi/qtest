#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IRQ {
    pub line: u32,

    pub state: IRQState,
}

impl IRQ {
    pub fn new(line: u32, state: IRQState) -> Self {
        IRQ {
            line: line,
            state: state,
        }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IRQState {
    Raise,
    Lower,
}
