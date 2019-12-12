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

/*

[src\solutions\day9.rs:11] instructions = [
    Mul 34463338i 34463338i 63,
    Lst 63p 34463338i 63,
    JmT 63p 53i,
    Mul 3i 1i 1000,
    Srb 988i,
    Srb 12r,
    Srb 1000p,
    Srb 6r,
    Srb 3r,
    Inp 0,
    Eqt 1000p 1i 63,
    JmT 63p 65i,
    Eqt 1000p 2i 63,
    JmT 63p 904i,
    Eqt 1000p 0i 63,
    JmT 63p 58i,
    *Out 25p,
    Out 0i,
    Halt,
]

rel: 1000

*/