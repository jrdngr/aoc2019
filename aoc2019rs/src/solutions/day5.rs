use anyhow::Result;

use crate::utils;
use crate::intcode::IntcodeMachine;

pub fn run() -> Result<String> {
    let program = utils::read_input_list_as::<i64>(5, b',')?;

    let mut machine = IntcodeMachine::new(&program);
    machine.run()?;

    Ok(String::from(""))
}
