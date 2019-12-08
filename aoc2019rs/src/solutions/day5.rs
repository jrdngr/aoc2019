use anyhow::Result;

use crate::utils::input;
use crate::utils::intcode::IntcodeMachine;

// Part 1: 9025675
// Part 2: 11981754

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(5, b',')?;

    let mut machine = IntcodeMachine::new_console_machine(&program);
    machine.run()?;

    Ok(String::from("See last output"))
}
