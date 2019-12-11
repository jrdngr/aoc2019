use anyhow::Result;

use crate::utils::input;
use crate::intcode::helpers;

// Part 1: 9025675
// Part 2: 11981754

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(5, b',')?;

    let part1 = helpers::process_input_last_output(&program, &[1]).unwrap();
    let part2 = helpers::process_input_last_output(&program, &[5]).unwrap();


    Ok(format!("Part 1: {}\nPart 2: {}", part1, part2))
}
