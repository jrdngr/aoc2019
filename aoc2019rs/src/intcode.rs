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
    use super::*;

    fn test_program(program: &[i64]) -> Vec<i64> {
        let mut machine = IntcodeMachine::new_console_machine(&program);
        machine.run();
        machine.memory().to_vec()
    }

    #[test]
    fn day2_tests() {
        assert_eq!(test_program(&[1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(test_program(&[2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(test_program(&[2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(test_program(&[1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }

    #[test]
    fn day2_part1() {
        let input = day2_input();
        assert_eq!(run_day2_test(&input, 12, 2), 7594646);
    }

    #[test]
    fn day2_part2() {
        let input = day2_input();
        for noun in 0..=99 {
            for verb in 0..=99 {
                let result = run_day2_test(&input, noun, verb);
                if result == 19690720 {
                    return assert_eq!(100 * noun + verb, 3376);
                }
                
            }
        }

        assert!(false)
    }

    fn day2_input() -> Vec<i64> {
        crate::utils::input::read_input_list_as::<i64>(2, b',').unwrap()
    }

    fn run_day2_test(program: &[i64], noun: i64, verb: i64) -> i64 {
        let mut machine = IntcodeMachine::new_console_machine(program);
        machine.write_memory(1, noun);
        machine.write_memory(2, verb);
        machine.run();
    
        machine.read_memory_position(0)
    }

    #[test]
    fn day5_tests() {
        assert_eq!(test_program(&[1002,4,3,4,33]), vec![1002,4,3,4,99]);
        assert_eq!(test_program(&[1101,100,-1,4,0]), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn day5_part1() {
        let program = day5_input();
        let result = helpers::process_input_last_output(&program, &[1]);
        assert_eq!(result, Some(String::from("9025675")));
    }

    fn day5_input() -> Vec<i64> {
        crate::utils::input::read_input_list_as::<i64>(5, b',').unwrap()
    }

    #[test]
    fn day5_comparison_tests() {
        assert_eq!(helpers::process_input_last_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[7]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[8]), Some(String::from("1")));
        
        assert_eq!(helpers::process_input_last_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[7]), Some(String::from("1")));
        assert_eq!(helpers::process_input_last_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[9]), Some(String::from("0")));

        assert_eq!(helpers::process_input_last_output(&[3,3,1108,-1,8,3,4,3,99], &[7]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,3,1108,-1,8,3,4,3,99], &[8]), Some(String::from("1")));

        assert_eq!(helpers::process_input_last_output(&[3,3,1107,-1,8,3,4,3,99], &[7]), Some(String::from("1")));
        assert_eq!(helpers::process_input_last_output(&[3,3,1107,-1,8,3,4,3,99], &[9]), Some(String::from("0")));
    }

    #[test]
    fn day5_jump_position_tests() {
        assert_eq!(helpers::process_input_last_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[1]), Some(String::from("1")));
    }

    #[test]
    fn day5_jump_immediate_tests() {
        assert_eq!(helpers::process_input_last_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[1]), Some(String::from("1")));
    } 

    #[test]
    fn day5_complex_test() {
        assert_eq!(helpers::process_input_last_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[7]), Some(String::from("999")));

        assert_eq!(helpers::process_input_last_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[8]), Some(String::from("1000")));

        assert_eq!(helpers::process_input_last_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[9]), Some(String::from("1001")));
        
    }
}
