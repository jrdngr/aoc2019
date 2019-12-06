use anyhow::Result;

use crate::utils;
use crate::intcode::IntcodeMachine;

// Part 1: 9025675
// Part 2: 

pub fn run() -> Result<String> {
    let program = utils::read_input_list_as::<i64>(5, b',')?;

    let mut machine = IntcodeMachine::new(&program);
    machine.run()?;

    Ok(String::from("See last output"))
}
