use anyhow::{bail, Result};

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
            use IntcodeInstruction::*;
            
            match instruction {
                Add(x, y, position) => {
                    self.memory[position as usize] = x + y;
                    self.instruction_pointer += 4;
                },
                Multiply(x, y, position) => {
                    self.memory[position as usize] = x * y;
                    self.instruction_pointer += 4;
                }
                Input(position) => {
                    self.memory[position as usize] = self.input();
                    self.instruction_pointer += 2;
                },
                Output(position) => {
                    self.output(self.memory[position as usize]);
                    self.instruction_pointer += 2;
                },
                Halt => break,
            }
            
            self.instruction_pointer += 4;
        }

        Ok(())
    }

    pub fn read_memory(&self, position: i64) -> i64 {
        self.memory[position as usize]
    }

    pub fn write_memory(&mut self, position: i64, value: i64) {
        self.memory[position as usize] = value;
    }

    fn read_instruction(&self) -> Result<IntcodeInstruction> {
        if self.instruction_pointer >= self.memory.len() {
            bail!("Instruction pointer out of range")
        } else {
            use IntcodeInstruction::*;

            let mem = &self.memory;
            let ptr = self.instruction_pointer;
            
            Ok(match mem[ptr] {
                1 => Add(mem[mem[ptr + 1] as usize], mem[mem[ptr + 2] as usize], mem[ptr + 3]),
                2 => Multiply(mem[mem[ptr + 1] as usize], mem[mem[ptr + 2] as usize], mem[ptr + 3]),
                3 => Input(mem[ptr + 1]),
                4 => Output(mem[ptr + 1]),
                99 => Halt,
                _ => bail!("Invalid instruction: {}", mem[ptr]),
            })
        }
    }

    fn input(&self) -> i64 {
        0
    }

    fn output(&self, value: i64) {
        println!("{}", value);
    }
}

pub enum IntcodeInstruction {
    Add(i64, i64, i64),
    Multiply(i64, i64, i64),
    Input(i64),
    Output(i64),
    Halt,
}