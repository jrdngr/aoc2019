use anyhow::Result;

use crate::utils::input;
use crate::intcode::helpers;

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(9, b',')?;

    
    let(instructions, output) = helpers::debug_process_input(&program, &[1]);
    dbg!(instructions);
    dbg!(output);
    
    Ok(format!("Part 1: {:?}\nPart 2: {}\n", 0, 0))
}
