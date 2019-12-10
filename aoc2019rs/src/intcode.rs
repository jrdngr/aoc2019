use anyhow::{bail, Result};

mod operation;
mod instruction;
mod input;
mod output;

pub use self::operation::{Mode, MachineOperation};
pub use self::instruction::{IntcodeInstruction, NextStep};
pub use self::input::{IntcodeInput, IntcodeConsoleInput, IntcodePresetInput};
pub use self::output::{IntcodeOutput, IntcodeConsoleOutput, IntcodeHistoryOutput};

pub struct IntcodeMachine {
    instruction_pointer: usize,
    memory: Vec<i64>,
    input: Box<dyn IntcodeInput>,
    output: Box<dyn IntcodeOutput>,
}

impl IntcodeMachine {
    pub fn new(machine_code: &[i64], input: Box<dyn IntcodeInput>, output: Box<dyn IntcodeOutput>) -> Self {
        Self {
            instruction_pointer: 0,
            memory: machine_code.to_vec(),
            input,
            output,
        }
    }

    pub fn teardown(self) -> (Vec<i64>, Box<dyn IntcodeInput>, Box<dyn IntcodeOutput>) {
        (self.memory, self.input, self.output)
    }

    pub fn new_console_machine(machine_code: &[i64]) -> Self {
        Self::new(machine_code, Box::new(IntcodeConsoleInput), Box::new(IntcodeConsoleOutput::new()))
    }

    pub fn new_automated_machine(machine_code: &[i64], inputs: &[i64]) ->Self {
        let input_handler = Box::new(IntcodePresetInput::new(inputs));
        let output_handler = Box::new(IntcodeHistoryOutput::new());
        IntcodeMachine::new(machine_code, input_handler, output_handler)
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let instruction = self.read_instruction()?;
            match instruction.operate(self)? {
                NextStep::Skip(steps) => self.instruction_pointer += steps,
                NextStep::Jump(position) => self.instruction_pointer = position,
                NextStep::Halt => break,
            }
        }

        Ok(())
    }

    pub fn read_memory(&self, position: usize) -> i64 {
        self.memory[position]
    }

    pub fn read_slice(&self, range: std::ops::Range<usize>) -> &[i64] {
        &self.memory[range]
    }

    pub fn read_slice_from_ptr(&self, length: usize) -> &[i64] {
        let start = self.instruction_pointer;
        let end = start + length;
        self.read_slice(start..end)
    }

    pub fn write_memory(&mut self, position: usize, value: i64) {
        self.memory[position] = value;
    }

    #[cfg(test)]
    pub fn memory_as_slice(&self) -> &[i64] {
        &self.memory
    }

    pub fn get_value(&self, mode: Mode, value: i64) -> i64 {
        match mode {
            Mode::Position => self.read_memory(value as usize),
            Mode::Immediate => value as i64,
        }
    }

    pub fn input(&mut self) -> Result<i64> {
        self.input.process()
    }

    pub fn output(&mut self, value: i64) {
        self.output.process(value)
    }

    fn read_instruction(&self) -> Result<IntcodeInstruction> {
        if self.instruction_pointer >= self.memory.len() {
            bail!("Instruction pointer out of range")
        } else {
            let mem = &self.memory;
            let ptr = self.instruction_pointer;
            let operation = MachineOperation::new(mem[ptr])?;
            IntcodeInstruction::new(operation, self)
        }
    }
}

// Special run functions
impl IntcodeMachine {
    pub fn process_input(program: &[i64], inputs: &[i64]) -> Vec<String> {
        let mut machine = IntcodeMachine::new_automated_machine(program, inputs);
        machine.run().unwrap();
        let (_, _, output_handler) = machine.teardown();
        output_handler.history().to_vec()
    }

    pub fn process_input_single_output(program: &[i64], inputs: &[i64]) -> Option<String> {
        Self::process_input(program, inputs).last().cloned()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_program(program: &[i64]) -> Vec<i64> {
        let mut machine = IntcodeMachine::new_console_machine(&program);
        machine.run().unwrap();
        machine.memory_as_slice().to_vec()
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
        assert_eq!(run_day2_test(&input, 12, 2).unwrap(), 7594646);
    }

    #[test]
    fn day2_part2() {
        let input = day2_input();
        for noun in 0..=99 {
            for verb in 0..=99 {
                let result = run_day2_test(&input, noun, verb).unwrap();
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

    fn run_day2_test(program: &[i64], noun: i64, verb: i64) -> Result<i64> {
        let mut machine = IntcodeMachine::new_console_machine(program);
        machine.write_memory(1, noun);
        machine.write_memory(2, verb);
        machine.run()?;
    
        Ok(machine.read_memory(0))
    }

    #[test]
    fn day5_tests() {
        assert_eq!(test_program(&[1002,4,3,4,33]), vec![1002,4,3,4,99]);
        assert_eq!(test_program(&[1101,100,-1,4,0]), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn day5_part1() {
        let program = day5_input();
        let result = IntcodeMachine::process_input_single_output(&program, &[1]);
        assert_eq!(result, Some(String::from("9025675")));
    }

    fn day5_input() -> Vec<i64> {
        crate::utils::input::read_input_list_as::<i64>(5, b',').unwrap()
    }

    #[test]
    fn day5_comparison_tests() {
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[7]), Some(String::from("0")));
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[8]), Some(String::from("1")));
        
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[7]), Some(String::from("1")));
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[9]), Some(String::from("0")));

        assert_eq!(IntcodeMachine::process_input_single_output(&[3,3,1108,-1,8,3,4,3,99], &[7]), Some(String::from("0")));
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,3,1108,-1,8,3,4,3,99], &[8]), Some(String::from("1")));

        assert_eq!(IntcodeMachine::process_input_single_output(&[3,3,1107,-1,8,3,4,3,99], &[7]), Some(String::from("1")));
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,3,1107,-1,8,3,4,3,99], &[9]), Some(String::from("0")));
    }

    #[test]
    fn day5_jump_position_tests() {
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0]), Some(String::from("0")));
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[1]), Some(String::from("1")));
    }

    #[test]
    fn day5_jump_immediate_tests() {
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0]), Some(String::from("0")));
        assert_eq!(IntcodeMachine::process_input_single_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[1]), Some(String::from("1")));
    } 

    #[test]
    fn day5_complex_test() {
        assert_eq!(IntcodeMachine::process_input_single_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[7]), Some(String::from("999")));

        assert_eq!(IntcodeMachine::process_input_single_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[8]), Some(String::from("1000")));

        assert_eq!(IntcodeMachine::process_input_single_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[9]), Some(String::from("1001")));
        
    }
}
