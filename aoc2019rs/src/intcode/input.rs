use anyhow::{bail, Result};

use std::str::FromStr;

use crate::utils::input;

pub trait IntcodeInput {
    fn process(&mut self) -> Result<i64>;
}

pub struct IntcodeConsoleInput;

impl IntcodeInput for IntcodeConsoleInput {
    fn process(&mut self) -> Result<i64> {
        let input = input::read_input()?;
        Ok(i64::from_str(&input)?)
    }
}

pub struct IntcodePresetInput {
    inputs: Box<dyn Iterator<Item=i64>>,
}

impl IntcodePresetInput {
    pub fn new(inputs: &[i64]) -> Self {
        Self { inputs: Box::new(inputs.to_vec().into_iter()) }
    }
}

impl IntcodeInput for IntcodePresetInput {
    fn process(&mut self) -> Result<i64> {
        match self.inputs.next() {
            Some(input) => Ok(input),
            None => bail!("Ran out of inputs"),
        }
    }
}
