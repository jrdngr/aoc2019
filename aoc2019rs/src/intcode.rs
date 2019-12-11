pub mod helpers;
mod instruction;
mod input;
mod output;

pub use self::instruction::IntcodeInstruction;
pub use self::input::{IntcodeInput, IntcodeConsoleInput, IntcodePresetInput};
pub use self::output::{IntcodeOutput, IntcodeConsoleOutput, IntcodeHistoryOutput};

#[derive(Debug, PartialEq)]
pub enum IntcodeState {
    Initialized,
    Running,
    Halting,
    WaitingForInput,
}

pub struct IntcodeMachine<I, O> {
    state: IntcodeState,
    instruction_pointer: usize,
    memory: Vec<i64>,
    input_handler: I,
    output_handler: O,
}

impl<I, O> IntcodeMachine<I, O>
where I: IntcodeInput,
      O: IntcodeOutput,
{
    pub fn new(machine_code: &[i64], input_handler: I, output_handler: O) -> Self {
        Self {
            state: IntcodeState::Initialized,
            instruction_pointer: 0,
            memory: machine_code.to_vec(),
            input_handler,
            output_handler,
        }
    }

    pub fn teardown(self) -> (IntcodeState, Vec<i64>, I, O) {
        (self.state, self.memory, self.input_handler, self.output_handler)
    }

    pub fn run(&mut self) {
        while self.state != IntcodeState::Halting {
            self.run_next_instruction();
        }
    }

    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn read_memory_position(&self, position: usize) -> i64 {
        self.memory[position]
    }

    pub fn write_memory(&mut self, position: usize, value: i64) {
        self.memory[position] = value;
    }

    pub fn input(&mut self) -> i64 {
        self.input_handler.process().expect("Error processing input")
    }

    pub fn output(&mut self, value: i64) {
        self.output_handler.process(value)
    }

    pub fn input_handler(&self) -> &I {
        &self.input_handler
    }

    pub fn output_handler(&self) -> &O {
        &self.output_handler
    }

    fn run_next_instruction(&mut self) {
        if self.instruction_pointer >= self.memory.len() {
            panic!("Instruction pointer out of range")
        } else {
            let ptr = self.instruction_pointer;
            let opcode = self.memory[ptr];
            let instruction = IntcodeInstruction::new(opcode, &self.memory[ptr..ptr+4]);
            self.operate(instruction);
        }
    }

    fn operate(&mut self, instruction: IntcodeInstruction) {
        use IntcodeInstruction::*;
        match instruction {
            Add{x, y, position} => {
                let x = x.evaluate(&self.memory);
                let y = y.evaluate(&self.memory);
                self.write_memory(position, x + y);
                self.instruction_pointer += 4;
            },
            Multiply{x, y, position} => {
                let x = x.evaluate(&self.memory);
                let y = y.evaluate(&self.memory);
                self.write_memory(position, x * y);
                self.instruction_pointer += 4;
            },
            Input{position} => {
                let input = self.input();
                self.write_memory(position, input);
                self.instruction_pointer += 2;
            },
            Output{value} => {
                self.output(value.evaluate(&self.memory));
                self.instruction_pointer += 2;
            },
            JumpIfTrue{test_position, jump_position} => {
                let test_value = test_position.evaluate(&self.memory);
                if test_value > 0 {
                    self.instruction_pointer = jump_position.evaluate(&self.memory) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            },
            JumpIfFalse{test_position, jump_position} => {
                let test_value = test_position.evaluate(&self.memory);
                if test_value == 0 {
                    self.instruction_pointer = jump_position.evaluate(&self.memory) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            },
            IsLessThan{x, y, position} => {
                let x = x.evaluate(&self.memory);
                let y = y.evaluate(&self.memory);
                if x < y {
                    self.write_memory(position, 1);
                } else {
                    self.write_memory(position, 0);
                }
                self.instruction_pointer += 4;
            },
            IsEquals{x, y, position} => {
                let x = x.evaluate(&self.memory);
                let y = y.evaluate(&self.memory);
                if x == y {
                    self.write_memory(position, 1);
                } else {
                    self.write_memory(position, 0);
                }
                self.instruction_pointer += 4;
            },
            Halt => self.state = IntcodeState::Halting,
        }     
    }
}

impl IntcodeMachine<IntcodeConsoleInput, IntcodeConsoleOutput> {
    pub fn new_console_machine(machine_code: &[i64]) -> IntcodeMachine<IntcodeConsoleInput, IntcodeConsoleOutput> {
        IntcodeMachine::new(machine_code, IntcodeConsoleInput, IntcodeConsoleOutput::new())
    }
}

impl IntcodeMachine<IntcodePresetInput, IntcodeHistoryOutput> {
    pub fn new_automated_machine(machine_code: &[i64], inputs: &[i64]) -> IntcodeMachine<IntcodePresetInput, IntcodeHistoryOutput> {
        let input_handler = IntcodePresetInput::new(inputs);
        let output_handler = IntcodeHistoryOutput::new();
        IntcodeMachine::new(machine_code, input_handler, output_handler)
    }
}

#[cfg(test)]
mod tests {
    // Write me next!
}
