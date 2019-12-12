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

    fn debug_next_instruction(&mut self) {
        if self.instruction_pointer >= self.memory.len() {
            panic!("Instruction pointer out of range")
        } else {
            let ptr = self.instruction_pointer;
            let opcode = self.memory[ptr];
            let instruction = IntcodeInstruction::new(opcode, &self.memory[ptr+1..]);
            dbg!(&self);
            dbg!(&instruction);
            let input = crate::utils::input::read_input_with_prompt("").unwrap();
            if input != "" {
                panic!("Aborting execution");
            }            
            self.operate(instruction);
        }        
    }

    fn operate(&mut self, instruction: IntcodeInstruction) {
        use IntcodeInstruction::*;
        
        match instruction {
            Add{x, y, position} => {
                let x = x.evaluate(&self.memory, self.relative_base);
                let y = y.evaluate(&self.memory, self.relative_base);
                self.write_memory(position, x + y);
                self.instruction_pointer += 4;
            },
            Multiply{x, y, position} => {
                let x = x.evaluate(&self.memory, self.relative_base);
                let y = y.evaluate(&self.memory, self.relative_base);
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
                self.process_output(value.evaluate(&self.memory, self.relative_base));
                self.instruction_pointer += 2;
            },
            JumpIfTrue{test_position, jump_position} => {
                let test_value = test_position.evaluate(&self.memory, self.relative_base);
                if test_value > 0 {
                    self.instruction_pointer = jump_position.evaluate(&self.memory, self.relative_base) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            },
            JumpIfFalse{test_position, jump_position} => {
                let test_value = test_position.evaluate(&self.memory, self.relative_base);
                if test_value == 0 {
                    self.instruction_pointer = jump_position.evaluate(&self.memory, self.relative_base) as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            },
            IsLessThan{x, y, position} => {
                let x = x.evaluate(&self.memory, self.relative_base);
                let y = y.evaluate(&self.memory, self.relative_base);
                if x < y {
                    self.write_memory(position, 1);
                } else {
                    self.write_memory(position, 0);
                }
                self.instruction_pointer += 4;
            },
            IsEquals{x, y, position} => {
                let x = x.evaluate(&self.memory, self.relative_base);
                let y = y.evaluate(&self.memory, self.relative_base);
                if x == y {
                    self.write_memory(position, 1);
                } else {
                    self.write_memory(position, 0);
                }
                self.instruction_pointer += 4;
            }, 
            SetRelativeBase{offset} => {
                self.relative_base += offset.evaluate(&self.memory, self.relative_base) as usize;
                self.instruction_pointer += 2;
            },
            Halt => self.state = IntcodeState::Halted,
        }     
    }
}

impl<I, O> std::fmt::Debug for IntcodeMachine<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
               "IntcodeMachine {{\n  state: {:?}\n  instruction_ptr: {}\n  relative_base: {}\n  memory: {:?}\n}}\n", 
               &self.state, 
               self.instruction_pointer,
                self.relative_base, 
                &self.memory[..50])
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
        machine.memory()[0..program.len()].to_vec()
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

    #[test]
    fn test_relative_base() {
        use std::str::FromStr;

        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let result = helpers::process_input(&program, &[]).into_iter()
            .flat_map(|output| i64::from_str(&output))
            .collect::<Vec<i64>>();
     
        assert_eq!(&result, &program);
    }
}

/*
    Mode:
    0 => Position,
    1 => Immediate,
    2 => Relative,

    Opcode:
    1 => Add(x, y, pos)
    2 => Multiply(x, y, pos)
    3 => Input(pos)
    4 => Output(value) 
    5 => JumpIfTrue(test, jump)
    6 => JumpIfFalse(test, jump)
    7 => IsLessThan(x, y, pos)
    8 => IsEquals(x, y, pos)
    9 => SetRelativeBase(value)
    99 => Halt,



    [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]

    SetRel(i(1)) = SetRel(1)
    Output(r(-1)) = Output(0)
    Add(p(100),i(1),p(100)) = Add(0, 1, 0)
    [1, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    IsEq(p(100),i(16),p(101)) = IsEq(0,16,0)
    [0, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    JmpF(p(101),i(0)) = JmpF(0,0)
    Halt
*/

