use crate::utils::conversion;

#[derive(Debug, PartialEq)]
pub enum IntcodeInstruction {
    Add{x: IntcodeValue, y: IntcodeValue, position: usize},
    Multiply{x: IntcodeValue, y: IntcodeValue, position: usize},
    Input{position: usize},
    Output{value: IntcodeValue},
    JumpIfTrue{test_position: IntcodeValue, jump_position: IntcodeValue},
    JumpIfFalse{test_position: IntcodeValue, jump_position: IntcodeValue},
    IsLessThan{x: IntcodeValue, y: IntcodeValue, position: usize},
    IsEquals{x: IntcodeValue, y: IntcodeValue, position: usize},
    Halt,
}

impl IntcodeInstruction {
    pub fn new(opcode: i64, params: &[i64]) -> Self {
        use IntcodeInstruction::*;

        let digits: Vec<usize> = conversion::i64_into_digits(&opcode);
        let get_value = |param_position| {
            let mode = *digits.get(param_position + 1).unwrap_or(&0);
            match mode {
                0 => IntcodeValue::Position(params[param_position] as usize),
                1 => IntcodeValue::Immediate(params[param_position]),
                _ => panic!("Invalid parameter mode: {}", mode),
            }
        };

        match opcode {
            1 => {
                Add {
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }
            },
            2 =>  {
                Multiply{ 
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }                
            },
            3 =>  {
                Input{ position: params[0] as usize }
            },
            4 =>  {
                Output{ 
                    value: get_value(0)
                }
            },
            5 => {
                JumpIfTrue { 
                    test_position: get_value(0),
                    jump_position: get_value(1),
                }
            },
            6 => {
                JumpIfFalse { 
                    test_position: get_value(0),
                    jump_position: get_value(1),
                }
            },
            7 => {
                IsLessThan {
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }
            },
            8 => {
                IsEquals {
                    x: get_value(0),
                    y: get_value(1),
                    position: params[2] as usize,
                }
            },
            99 => Halt,
            _ => panic!("Invalid instruction: {:?}", opcode),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum IntcodeValue {
    Position(usize),
    Immediate(i64),
}

impl IntcodeValue {
    pub fn new(mode: usize, value: i64) -> Self {
        match value {
            0 => IntcodeValue::Position(value as usize),
            1 => IntcodeValue::Immediate(value),
            _ => panic!("Invalid mode code: {}", mode),
        }
    }

    pub fn evaluate(&self, memory: &[i64]) -> i64 {
        match self {
            IntcodeValue::Position(position) => memory[*position],
            IntcodeValue::Immediate(value) => *value,
        }
    }
}
