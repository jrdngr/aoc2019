#[derive(Clone, PartialEq)]
pub enum IntcodeValue {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

impl IntcodeValue {
    pub fn evaluate(&self, memory: &[i64], relative_base: usize) -> i64 {
        match self {
            IntcodeValue::Position(position) => memory[*position],
            IntcodeValue::Immediate(value) => *value,
            IntcodeValue::Relative(offset) => memory[(relative_base as i64 + offset) as usize],
        }
    }
}

impl std::fmt::Debug for IntcodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use IntcodeValue::*;
        
        let (postfix, value) = match self {
            Position(value) => ("p", *value as i64),
            Immediate(value) => ("i", *value),
            Relative(value) => ("r", *value),
        };
        
        write!(f, "{}{}", value, postfix)
    }
}
