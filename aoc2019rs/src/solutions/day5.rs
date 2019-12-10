use anyhow::Result;

use crate::utils::input;
use crate::utils::intcode::IntcodeMachine;

// Part 1: 9025675
// Part 2: 11981754

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(5, b',')?;

    let part1 = IntcodeMachine::process_input_single_output(&program, &[1]).unwrap();
    let part2 = IntcodeMachine::process_input_single_output(&program, &[5]).unwrap();


    Ok(format!("Part 1: {}\nPart 2: {}", part1, part2))
}
