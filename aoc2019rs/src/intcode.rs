pub mod helpers;
mod instruction;
mod input;
mod output;

pub use self::instruction::IntcodeInstruction;
pub use self::input::{IntcodeInput, IntcodeConsoleInput, IntcodePresetInput, IntcodeBlockingInput};
pub use self::output::{IntcodeOutput, IntcodeConsoleOutput, IntcodeHistoryOutput};

#[derive(Debug, PartialEq)]
pub enum IntcodeState {
    Initialized,
    Running,
    Suspended,
    Halted,
}

pub struct IntcodeMachine<I, O> {
    state: IntcodeState,
    instruction_pointer: usize,
    relative_base: usize,
    memory: Vec<i64>,
    input_handler: I,
    output_handler: O,
}

impl<I, O> IntcodeMachine<I, O>
where I: IntcodeInput,
      O: IntcodeOutput,
{
    pub fn new(machine_code: &[i64], input_handler: I, output_handler: O) -> Self {
        let mut memory = vec![0; 1024];
        for i in 0..machine_code.len() {
            memory[i] = machine_code[i];
        }
        
        Self {
            state: IntcodeState::Initialized,
            instruction_pointer: 0,
            relative_base: 0,
            memory,
            input_handler,
            output_handler,
        }
    }

    pub fn run(&mut self) {
        self.state = IntcodeState::Running;
        while self.state == IntcodeState::Running {
            self.run_next_instruction();
        }
    }
    
    pub fn teardown(self) -> (IntcodeState, Vec<i64>, I, O) {
        (self.state, self.memory, self.input_handler, self.output_handler)
    }

    pub fn state(&self) -> &IntcodeState {
        &self.state
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

    pub fn input(&mut self, value: i64) {
        let position = self.memory[self.instruction_pointer + 1] as usize;
        self.write_memory(position, value);
        self.instruction_pointer += 2;
    }

    pub fn process_input(&mut self) -> Option<i64> {
        self.input_handler.process()
    }

    pub fn process_output(&mut self, value: i64) {
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
            let instruction = IntcodeInstruction::new(opcode, &self.memory[ptr+1..]);
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
                match self.process_input() {
                    Some(input) => { 
                        self.write_memory(position, input);
                        self.instruction_pointer += 2;
                    },
                    None => self.state = IntcodeState::Suspended,
                }
            },
            Output{value} => {
                self.process_output(value.evaluate(&self.memory));
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
            SetRelativeBase{value} => self.relative_base = value.evaluate(&self.memory) as usize,
            Halt => self.state = IntcodeState::Halted,
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
        IntcodeMachine::new(machine_code, IntcodePresetInput::new(inputs), IntcodeHistoryOutput::new())
    }
}

impl IntcodeMachine<IntcodeBlockingInput, IntcodeHistoryOutput> {
    pub fn new_blocking_machine(machine_code: &[i64]) -> IntcodeMachine<IntcodeBlockingInput, IntcodeHistoryOutput> {
        IntcodeMachine::new(machine_code, IntcodeBlockingInput, IntcodeHistoryOutput::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_program(program: &[i64]) -> Vec<i64> {
        let mut machine = IntcodeMachine::new_automated_machine(&program, &[]);
        machine.run();
        machine.memory().to_vec()
    }

    #[test]
    fn test_add() {
        assert_eq!(test_program(&[1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(test_program(&[1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(test_program(&[2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(test_program(&[2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn test_add_with_modes() {
        assert_eq!(test_program(&[1002,4,3,4,33]), vec![1002,4,3,4,99]);
    }

    #[test]
    fn test_multiply_with_modes() {
        assert_eq!(test_program(&[1101,100,-1,4,0]), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_chaining() {
        use std::str::FromStr;

        let program = vec![
            3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
        ];
        let phases = vec![9,8,7,6,5];

        let mut amplifiers = vec![
            IntcodeMachine::new_blocking_machine(&program),
            IntcodeMachine::new_blocking_machine(&program),
            IntcodeMachine::new_blocking_machine(&program),
            IntcodeMachine::new_blocking_machine(&program),
            IntcodeMachine::new_blocking_machine(&program),
        ];
    
        // Initialize with phase
        for (i, amp) in amplifiers.iter_mut().enumerate() {
            amp.run();
            amp.input(phases[i]);
        }
    
        let mut next_input = 0;
        loop {
            for amp in amplifiers.iter_mut() {
                amp.run();
                amp.input(next_input);
                amp.run();
    
                let output = amp.output_handler()
                    .last_output()
                    .expect("No output available")
                    .to_owned();
    
                next_input = i64::from_str(&output).unwrap();
            }
            if amplifiers[4].state() == &IntcodeState::Halted {
                break;
            }
        }
    
        let last_output = amplifiers[4].output_handler().last_output().unwrap().to_owned();
        assert_eq!(last_output, "139629729");
    }
}
