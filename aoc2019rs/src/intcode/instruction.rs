use anyhow::{bail, Result};

use super::IntcodeMachine;
use super::operation::MachineOperation;

pub enum NextStep {
    Skip(usize),
    Jump(usize),
    Halt,
}

#[derive(Debug, PartialEq)]
pub enum IntcodeInstruction {
    Add{x: i64, y: i64, position: usize},
    Multiply{x: i64, y: i64, position: usize},
    Input{position: usize},
    Output{value: i64},
    JumpIfTrue{should_jump: bool, position: usize},
    JumpIfFalse{should_jump: bool, position: usize},
    IsLessThan{x: i64, y: i64, position: usize},
    IsEquals{x: i64, y: i64, position: usize},
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
                Output{ 
                    value: machine.get_value(operation.param1_mode, params[1])
                }
            },
            5 => {
                let params = machine.read_slice_from_ptr(3);
                let test_value = machine.get_value(operation.param1_mode, params[1]);
                JumpIfTrue { 
                    should_jump: test_value > 0, 
                    position: machine.get_value(operation.param2_mode, params[2]) as usize,
                }
            },
            6 => {
                let params = machine.read_slice_from_ptr(3);
                let test_value = machine.get_value(operation.param1_mode, params[1]);
                JumpIfFalse { 
                    should_jump: test_value == 0, 
                    position: machine.get_value(operation.param2_mode, params[2]) as usize,
                }
            },
            7 => {
                let params = machine.read_slice_from_ptr(4);
                IsLessThan {
                    x: machine.get_value(operation.param1_mode, params[1]),
                    y: machine.get_value(operation.param2_mode, params[2]),
                    position: params[3] as usize,
                }
            },
            8 => {
                let params = machine.read_slice_from_ptr(4);
                IsEquals {
                    x: machine.get_value(operation.param1_mode, params[1]),
                    y: machine.get_value(operation.param2_mode, params[2]),
                    position: params[3] as usize,
                }
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
                NextStep::Skip(4)
            },
            Multiply{x, y, position} => {
                machine.write_memory(*position, x * y);
                NextStep::Skip(4)
            },
            Input{position} => {
                let input = machine.input()?;
                machine.write_memory(*position, input);
                NextStep::Skip(2)
            },
            Output{value} => {
                machine.output(*value);
                NextStep::Skip(2)
            },
            JumpIfTrue{should_jump, position} => {
                if *should_jump {
                    NextStep::Jump(*position)
                } else {
                    NextStep::Skip(3)
                }
            },
            JumpIfFalse{should_jump, position} => {
                if *should_jump {
                    NextStep::Jump(*position)
                } else {
                    NextStep::Skip(3)
                }
            },
            IsLessThan{x, y, position} => {
                if x < y {
                    machine.write_memory(*position, 1);
                } else {
                    machine.write_memory(*position, 0);
                }
                NextStep::Skip(4)
            },
            IsEquals{x, y, position} => {
                if x == y {
                    machine.write_memory(*position, 1);
                } else {
                    machine.write_memory(*position, 0);
                }
                NextStep::Skip(4)
            },
            Halt => NextStep::Halt,
        })
    }
}