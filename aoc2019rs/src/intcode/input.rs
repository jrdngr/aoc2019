use std::str::FromStr;

use crate::utils::input;

pub trait IntcodeInput {
    fn process(&mut self) -> Option<i64>;
}

pub struct IntcodeConsoleInput;

impl IntcodeInput for IntcodeConsoleInput {
    fn process(&mut self) -> Option<i64> {
        let input = input::read_input().expect("Error reading input");
        Some(i64::from_str(&input).expect("Error parsing input"))
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
    fn process(&mut self) -> Option<i64> {
        Some(self.inputs.next().expect("Ran out of inputs"))
    }
}

pub struct IntcodeBlockingInput;

impl IntcodeInput for IntcodeBlockingInput {
    fn process(&mut self) -> Option<i64> {
        None
    }
}
