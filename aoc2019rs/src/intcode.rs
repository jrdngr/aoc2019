use anyhow::{bail, Result};

use std::convert::TryFrom;
use std::str::FromStr;

use crate::utils;

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

    pub fn new_console_machine(machine_code: &[i64]) -> Self {
        Self::new(machine_code, Box::new(IntcodeConsoleInput), Box::new(IntcodeConsoleOutput))
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            let instruction = self.read_instruction()?;
            match instruction.operate(self)? {
                NextStep::Jump(steps) => self.instruction_pointer += steps,
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

    pub fn input(&self) -> Result<i64> {
        self.input.process()
    }

    pub fn output(&self, value: i64) {
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

#[derive(Debug)]
pub struct MachineOperation {
    pub opcode: usize,
    pub param1_mode: Mode,
    pub param2_mode: Mode,
    pub param3_mode: Mode,
}

impl MachineOperation {
    pub fn new(instruction: i64) -> Result<Self> {
        let digits: Vec<usize> = utils::i64_into_digits(&instruction)
        .into_iter()
        .rev()
        .collect();

        if digits.is_empty() {
            bail!("Failed to split digits");
        }

        Ok(MachineOperation {
            opcode: digits[0] + 10 * digits.get(1).unwrap_or(&0),
            param1_mode: Mode::try_from(*digits.get(2).unwrap_or(&0))?,
            param2_mode: Mode::try_from(*digits.get(3).unwrap_or(&0))?,
            param3_mode: Mode::try_from(*digits.get(4).unwrap_or(&0))?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum IntcodeInstruction {
    Add{x: i64, y: i64, position: usize},
    Multiply{x: i64, y: i64, position: usize},
    Input{position: usize},
    Output{position: usize},
    Halt,
}

impl IntcodeInstruction {
    pub fn new(operation: MachineOperation, machine: &IntcodeMachine) -> Result<Self> {
        use IntcodeInstruction::*;
        Ok(match operation.opcode {
            1 => {
                let params = machine.read_slice_from_ptr(4);
                Add {
                    x: machine.get_value(operation.param1_mode, params[1]),
                    y: machine.get_value(operation.param2_mode, params[2]),
                    position: params[3] as usize,
                }
            },
            2 =>  {
                let params = machine.read_slice_from_ptr(4);
                Multiply{ 
                    x: machine.get_value(operation.param1_mode, params[1]),
                    y: machine.get_value(operation.param2_mode, params[2]),
                    position: params[3] as usize,
                }                
            },
            3 =>  {
                let params = machine.read_slice_from_ptr(2);
                Input{ position: params[1] as usize }
            },
            4 =>  {
                let params = machine.read_slice_from_ptr(2);
                Output{ position: params[1] as usize }
            },
            99 => Halt,
            _ => bail!("Invalid instruction: {:?}", operation),
        })
    }

    pub fn operate(&self, machine: &mut IntcodeMachine) -> Result<NextStep> {
        use IntcodeInstruction::*;
        Ok(match self {
            Add{x, y, position} => {
                machine.write_memory(*position, x + y);
                NextStep::Jump(4)
            },
            Multiply{x, y, position} => {
                machine.write_memory(*position, x * y);
                NextStep::Jump(4)
            },
            Input{position} => {
                let input = machine.input()?;
                machine.write_memory(*position, input);
                NextStep::Jump(2)
            },
            Output{position} => {
                let value = machine.read_memory(*position);
                machine.output(value);
                NextStep::Jump(2)
            },
            Halt => NextStep::Halt,
        })
    }
}

pub enum NextStep {
    Jump(usize),
    Halt,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<usize> for Mode {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self> {
        Ok(match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => bail!("Invalid mode code: {}", value),
        })
    }
}

pub trait IntcodeInput {
    fn process(&self) -> Result<i64>;
}

pub trait IntcodeOutput {
    fn process(&self, value: i64);
}

pub struct IntcodeConsoleInput;

impl IntcodeInput for IntcodeConsoleInput {
    fn process(&self) -> Result<i64> {
        let input = utils::read_input()?;
        Ok(i64::from_str(&input)?)
    }
}

pub struct IntcodeConsoleOutput;

impl IntcodeOutput for IntcodeConsoleOutput {
    fn process(&self, value: i64) {
        println!("Output: {}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;

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
        utils::read_input_list_as::<i64>(2, b',').unwrap()
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

    }
}
