use anyhow::{bail, Result};

use std::convert::TryFrom;
use std::str::FromStr;

use crate::utils;

pub struct IntcodeMachine {
    instruction_pointer: usize,
    memory: Vec<i64>,
}

impl IntcodeMachine {
    pub fn new(machine_code: &[i64]) -> Self {
        Self {
            instruction_pointer: 0,
            memory: machine_code.to_vec(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while let Ok(instruction) = self.read_instruction() {
            instruction.operate(self)?;
            self.instruction_pointer += instruction.length();
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
        let input = utils::read_input()?;
        Ok(i64::from_str(&input)?)
    }

    pub fn output(&self, value: i64) {
        println!("{}", value);
    }

    fn read_instruction(&self) -> Result<Box<dyn IntcodeInstruction>> {
        if self.instruction_pointer >= self.memory.len() {
            bail!("Instruction pointer out of range")
        } else {
            let mem = &self.memory;
            let ptr = self.instruction_pointer;
            let operation = MachineOperation::new(mem[ptr])?;
            
            Ok(match operation.opcode {
                1 => Box::new(Add::new(operation, self)),
                2 => Box::new(Multiply::new(operation, self)),
                3 => Box::new(Input::new(operation, self)),
                4 => Box::new(Output::new(operation, self)),
                99 => Box::new(Halt),
                _ => bail!("Invalid instruction: {}", mem[ptr]),
            })
        }
    }
}

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

pub trait IntcodeInstruction: std::fmt::Debug {
    fn operate(&self, machine: &mut IntcodeMachine) -> Result<()>;
    fn length(&self) -> usize;
}

#[derive(Debug)]
pub struct Halt;

impl IntcodeInstruction for Halt {
    fn operate(&self, _: &mut IntcodeMachine)  -> Result<()> { 
        Ok(())
    }
    fn length(&self) -> usize { 1 }
}

#[derive(Debug)]
pub struct Add {
    x: i64,
    y: i64,
    position: usize,
}

impl Add {
    pub fn new(op: MachineOperation, machine: &IntcodeMachine) -> Self {
        let params = machine.read_slice_from_ptr(4);
        Self {
            x: machine.get_value(op.param1_mode, params[1]),
            y: machine.get_value(op.param2_mode, params[2]),
            position: params[3] as usize,
        }
    }
}

impl IntcodeInstruction for Add {
    fn operate(&self, machine: &mut IntcodeMachine) -> Result<()> {
        machine.write_memory(self.position, self.x + self.y);
        Ok(())
    }

    fn length(&self) -> usize { 4 }
}

#[derive(Debug)]
pub struct Multiply {
    x: i64,
    y: i64,
    position: usize,
}

impl Multiply {
    pub fn new(op: MachineOperation, machine: &IntcodeMachine) -> Self {
        let params = machine.read_slice_from_ptr(4);
        Self {
            x: machine.get_value(op.param1_mode, params[1]),
            y: machine.get_value(op.param2_mode, params[2]),
            position: params[3] as usize,
        }
    }
}

impl IntcodeInstruction for Multiply {
    fn operate(&self, machine: &mut IntcodeMachine) -> Result<()> {
        machine.write_memory(self.position, self.x * self.y);
        Ok(())
    }
    
    fn length(&self) -> usize { 4 }
}

#[derive(Debug)]
pub struct Input {
    position: usize,
}

impl Input {
    pub fn new(op: MachineOperation, machine: &IntcodeMachine) -> Self {
        let params = machine.read_slice_from_ptr(2);
        Self {
            position: machine.get_value(op.param1_mode, params[1]) as usize,
        }
    }
}

impl IntcodeInstruction for Input {
    fn operate(&self, machine: &mut IntcodeMachine) -> Result<()> {
        let input = machine.input()?;
        machine.write_memory(self.position, input);
        Ok(())
    }
    
    fn length(&self) -> usize { 2 }
}

#[derive(Debug)]
pub struct Output {
    position: usize,
}

impl Output {
    pub fn new(op: MachineOperation, machine: &IntcodeMachine) -> Self {
        let params = machine.read_slice_from_ptr(2);
        Self {
            position: machine.get_value(op.param1_mode, params[1]) as usize,
        }
    }
}

impl IntcodeInstruction for Output {
    fn operate(&self, machine: &mut IntcodeMachine) -> Result<()> {
        let value = machine.read_memory(self.position);
        machine.output(value);
        Ok(())
    }

    fn length(&self) -> usize { 2 }
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


#[cfg(test)]
mod tests {
    use super::*;

    fn test_input(input: &[i64]) -> Vec<i64> {
        let mut machine = IntcodeMachine::new(&input);
        machine.run().unwrap();
        machine.memory_as_slice().to_vec()
    }

    #[test]
    fn day2_tests() {
        assert_eq!(test_input(&[1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(test_input(&[2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(test_input(&[2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(test_input(&[1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}
