use anyhow::{bail, Result};

use crate::utils;
use crate::intcode::IntcodeMachine;

pub fn run() -> Result<String> {
    let program = utils::read_input_list_as::<i64>(2, b',')?;
    
    let part_1_result = run_test(&program, 12, 2)?;
    dbg!(part_1_result);

    // for noun in 0..=99 {
    //     for verb in 0..=99 {
    //         let result = run_test(&program, noun, verb)?;
    //         if result == 19690720 {
    //             return Ok(format!("{}", 100 * noun + verb));
    //         }
            
    //     }
    // }

    bail!("Couldn't find inputs with output 19690720")
}

fn run_test(program: &[i64], noun: i64, verb: i64) -> Result<i64> {
    let mut machine = IntcodeMachine::new(&program);
    machine.write_memory(1, noun);
    machine.write_memory(2, verb);
    machine.run()?;

    Ok(machine.read_memory(0))
}

// Part 1: 7594646
// Part 2: 3376