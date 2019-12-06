use anyhow::{bail, Result};

use std::convert::TryFrom;

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
            // match instruction {
            //     Add(x, y, position) => {
            //         self.memory[position as usize] = x + y;
            //         self.instruction_pointer += 4;
            //     },
            //     Multiply(x, y, position) => {
            //         self.memory[position as usize] = x * y;
            //         self.instruction_pointer += 4;
            //     }
            //     Input(position) => {
            //         self.memory[position as usize] = self.input();
            //         self.instruction_pointer += 2;
            //     },
            //     Output(position) => {
            //         self.output(self.memory[position as usize]);
            //         self.instruction_pointer += 2;
            //     },
            //     Halt => break,
            // }
            
            self.instruction_pointer += 4;
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

    pub fn input(&self) -> i64 {
        0
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
            
            Ok(match mem[ptr] {
                // 1 => Add(mem[mem[ptr + 1] as usize], mem[mem[ptr + 2] as usize], mem[ptr + 3]),
                // 2 => Multiply(mem[mem[ptr + 1] as usize], mem[mem[ptr + 2] as usize], mem[ptr + 3]),
                // 3 => Input(mem[ptr + 1]),
                // 4 => Output(mem[ptr + 1]),
                // 99 => Halt,
                _ => bail!("Invalid instruction: {}", mem[ptr]),
            })
        }
    }
}

pub trait IntcodeInstruction {
    fn operate(&self, machine: &mut IntcodeMachine);
    fn instruction_length(&self) -> usize;
}

pub struct Halt;

impl IntcodeInstruction for Halt {
    fn operate(&self, _: &mut IntcodeMachine) { }
    fn instruction_length(&self) -> usize { 1 }
}

pub struct Add {
    x: i64,
    y: i64,
    position: usize,
}

impl Add {
    pub fn new(machine: &IntcodeMachine) -> Self {
        Self {
            x: 0,
            y: 0,
            position: 0,
        }
    }
}

impl IntcodeInstruction for Add {
    fn operate(&self, machine: &mut IntcodeMachine) {
        machine.write_memory(self.position, self.x + self.y);
    }

    fn instruction_length(&self) -> usize { 4 }
}

pub struct Multiply {
    x: i64,
    y: i64,
    position: usize,
}

impl Multiply {
    pub fn new(mode: Mode) -> Self {
        Self {
            x: 0,
            y: 0,
            position: 0,
        }
    }
}

impl IntcodeInstruction for Multiply {
    fn operate(&self, machine: &mut IntcodeMachine) {
        machine.write_memory(self.position, self.x * self.y);
    }
    
    fn instruction_length(&self) -> usize { 4 }
}

pub struct Input {
    position: usize,
}

impl Input {
    pub fn new(mode: Mode) -> Self {
        Self {
            position: 0,
        }
    }
}

impl IntcodeInstruction for Input {
    fn operate(&self, machine: &mut IntcodeMachine) {
        let input = machine.input();
        machine.write_memory(self.position, input);
    }
    
    fn instruction_length(&self) -> usize { 2 }
}

pub struct Output {
    position: usize,
}

impl Output {
    pub fn new(mode: Mode) -> Self {
        Self {
            position: 0,
        }
    }
}

impl IntcodeInstruction for Output {
    fn operate(&self, machine: &mut IntcodeMachine) {
        let value = machine.read_memory(self.position);
        machine.output(value);
    }

    fn instruction_length(&self) -> usize { 2 }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i64> for Mode {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self> {
        Ok(match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => bail!("Invalid mode code: {}", value),
        })
    }
}
