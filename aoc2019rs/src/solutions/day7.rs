use anyhow::Result;

use crate::utils::{input, math};
use crate::utils::intcode::IntcodeMachine;

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(7, b',')?;
    let phase_permutations = math::permutations(&[0, 1, 2, 3, 4]);

    let amp1_output = IntcodeMachine::process_input_single_output(&program.clone(), &[0, 0]);

    Ok(amp1_output.unwrap())
}

