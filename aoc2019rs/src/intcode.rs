use anyhow::{bail, Result};

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

    pub fn run(&mut self) -> Result<()> {
        while let Ok(instruction) = self.read_instruction() {
            use IntcodeInstruction::*;
            
            match instruction {
                Add(x, y, position) => self.memory[position] = x + y,
                Mul(x, y, position) => self.memory[position] = x * y,
                Halt => break,
            }
            
            self.instruction_pointer += 4;
        }

        Ok(())
    }

    pub fn read_memory(&self, position: usize) -> usize {
        self.memory[position]
    }

    pub fn write_memory(&mut self, position: usize, value: usize) {
        self.memory[position] = value;
    }

    fn read_instruction(&self) -> Result<IntcodeInstruction> {
        if self.instruction_pointer >= self.memory.len() {
            bail!("Instruction pointer out of range")
        } else {
            use IntcodeInstruction::*;

            let mem = &self.memory;
            let ptr = self.instruction_pointer;
            
            Ok(match mem[ptr] {
                1 => Add(mem[mem[ptr + 1]], mem[mem[ptr + 2]], mem[ptr + 3]),
                2 => Mul(mem[mem[ptr + 1]], mem[mem[ptr + 2]], mem[ptr + 3]),
                99 => Halt,
                _ => bail!("Invalid instruction: {}", mem[ptr]),
            })
        }
        
    }
}

pub enum IntcodeInstruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}