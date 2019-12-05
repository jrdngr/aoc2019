pub enum IntcodeInstruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

pub struct IntcodeMachine {
    instruction_pointer: usize,
    memory: Vec<usize>,
}

impl IntcodeMachine {
    pub fn new(machine_code: &[usize]) -> Self {
        Self {
            instruction_pointer: 0,
            memory: machine_code.to_vec(),
        }
    }
}
