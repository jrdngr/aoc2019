use anyhow::Result;

use std::str::FromStr;

use crate::utils::{input, math};
use crate::intcode::{IntcodeMachine, IntcodeState, IntcodeOutput, helpers};

// Part 1: 43812
// Part 2: 59597414

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(7, b',')?;

    Ok(format!("Part 1: {}\nPart 2: {}", run_day_1(&program), run_day_2(&program)))
}

fn run_day_1(program: &[i64]) -> i64 {
    let phase_permutations = math::permutations_cloned::<i64>(&[0, 1, 2, 3, 4]);

    phase_permutations.into_iter()
        .map(|permutation| run_day_1_phase_permutation(&program, &permutation))
        .max()
        .unwrap()
}

fn run_day_1_phase_permutation(program: &[i64], phases: &[i64]) -> i64 {
    let mut next_input = 0;
    for phase in phases {
        let output = helpers::process_input(&program.clone(), &[*phase, next_input])
            .last()
            .unwrap()
            .clone();
        next_input = i64::from_str(&output).unwrap();
    }

    next_input
}

fn run_day_2(program: &[i64]) -> i64 {
    let phase_permutations = math::permutations_cloned::<i64>(&[5, 6, 7, 8, 9]);
    
    phase_permutations.into_iter()
        .map(|permutation| run_day_2_phase_permutation(&program, &permutation))
        .max()
        .unwrap()
}

fn run_day_2_phase_permutation(program: &[i64], phases: &[i64]) -> i64 {
    let mut amplifiers = vec![
        IntcodeMachine::new_blocking_machine(&program),
        IntcodeMachine::new_blocking_machine(&program),
        IntcodeMachine::new_blocking_machine(&program),
        IntcodeMachine::new_blocking_machine(&program),
        IntcodeMachine::new_blocking_machine(&program),
    ];

    // Initialize with phase
    for (i, amp) in amplifiers.iter_mut().enumerate() {
        amp.run();
        amp.input(phases[i]);
    }

    let mut next_input = 0;
    loop {
        for amp in amplifiers.iter_mut() {
            amp.run();
            amp.input(next_input);
            amp.run();

            let output = amp.output_handler()
                .last_output()
                .expect("No output available")
                .to_owned();

            next_input = i64::from_str(&output).unwrap();
        }
        if amplifiers[4].state() == &IntcodeState::Halted {
            break;
        }
    }

    let last_output = amplifiers[4].output_handler().last_output().unwrap().to_owned();
    i64::from_str(&last_output).unwrap()
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn day7_part1_test() {
        let program = input::read_input_list_as::<i64>(7, b',').unwrap();
        let result = run_day_1(&program);
        assert_eq!(result, 43812);
    }
}
