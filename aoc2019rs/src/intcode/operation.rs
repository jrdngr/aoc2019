use anyhow::{bail, Result};

use std::convert::TryFrom;

use crate::utils::conversion;

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

#[derive(Debug)]
pub struct MachineOperation {
    pub opcode: usize,
    pub param1_mode: Mode,
    pub param2_mode: Mode,
    pub param3_mode: Mode,
}

impl MachineOperation {
    pub fn new(instruction: i64) -> Result<Self> {
        let digits: Vec<usize> = conversion::i64_into_digits(&instruction)
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
