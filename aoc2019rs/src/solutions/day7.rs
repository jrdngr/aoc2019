use anyhow::Result;

use std::str::FromStr;

use crate::utils::{input, math};
use crate::utils::intcode::IntcodeMachine;

// Part 1: 43812
// Part 2: 

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(7, b',')?;
    let phase_permutations = math::permutations_cloned::<i64>(&[0, 1, 2, 3, 4]);

    let day1 = phase_permutations.into_iter()
        .map(|permutation| run_phase_permutation(&program, &permutation))
        .max()
        .unwrap();

    Ok(format!("Part 1: {}\nPart 2: {}", day1, ""))
}

fn run_phase_permutation(program: &[i64], phases: &[i64]) -> i64 {
    let mut next_input = 0;
    for phase in phases {
        let output = IntcodeMachine::process_input_single_output(&program.clone(), &[*phase, next_input]).unwrap();
        next_input = i64::from_str(&output).unwrap();
    }

    next_input
}
