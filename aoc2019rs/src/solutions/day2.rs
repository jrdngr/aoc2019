use anyhow::Result;

use crate::utils;
use crate::intcode::IntcodeMachine;

pub fn run() -> Result<String> {
    let program = utils::read_input_list_as::<usize>(2, b',')?;
    
    let part_1_result = run_test(&program, 12, 2)?;

    Ok(format!("{}", part_1_result))
}

fn run_test(program: &[usize], input1: usize, input2: usize) -> Result<usize> {
    let mut machine = IntcodeMachine::new(&program);
    machine.write_memory(1, input1);
    machine.write_memory(2, input2);
    machine.run()?;

    Ok(machine.read_memory(0))
}

// Part 1: 7594646
// Part 2: 3376