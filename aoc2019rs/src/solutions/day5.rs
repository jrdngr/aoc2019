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

#[cfg(test)]
mod tests {
    use crate::intcode::{IntcodeMachine, helpers};

    fn test_program(program: &[i64]) -> Vec<i64> {
        let mut machine = IntcodeMachine::new_console_machine(&program);
        machine.run();
        machine.memory().to_vec()
    }

    #[test]
    fn day5_tests() {
        assert_eq!(test_program(&[1002,4,3,4,33]), vec![1002,4,3,4,99]);
        assert_eq!(test_program(&[1101,100,-1,4,0]), vec![1101,100,-1,4,99]);
    }

    #[test]
    fn day5_part1() {
        let program = day5_input();
        let result = helpers::process_input_last_output(&program, &[1]);
        assert_eq!(result, Some(String::from("9025675")));
    }

    fn day5_input() -> Vec<i64> {
        crate::utils::input::read_input_list_as::<i64>(5, b',').unwrap()
    }

    #[test]
    fn day5_comparison_tests() {
        assert_eq!(helpers::process_input_last_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[7]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,9,8,9,10,9,4,9,99,-1,8], &[8]), Some(String::from("1")));
        
        assert_eq!(helpers::process_input_last_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[7]), Some(String::from("1")));
        assert_eq!(helpers::process_input_last_output(&[3,9,7,9,10,9,4,9,99,-1,8], &[9]), Some(String::from("0")));

        assert_eq!(helpers::process_input_last_output(&[3,3,1108,-1,8,3,4,3,99], &[7]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,3,1108,-1,8,3,4,3,99], &[8]), Some(String::from("1")));

        assert_eq!(helpers::process_input_last_output(&[3,3,1107,-1,8,3,4,3,99], &[7]), Some(String::from("1")));
        assert_eq!(helpers::process_input_last_output(&[3,3,1107,-1,8,3,4,3,99], &[9]), Some(String::from("0")));
    }

    #[test]
    fn day5_jump_position_tests() {
        assert_eq!(helpers::process_input_last_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[1]), Some(String::from("1")));
    }

    #[test]
    fn day5_jump_immediate_tests() {
        assert_eq!(helpers::process_input_last_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0]), Some(String::from("0")));
        assert_eq!(helpers::process_input_last_output(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[1]), Some(String::from("1")));
    } 

    #[test]
    fn day5_complex_test() {
        assert_eq!(helpers::process_input_last_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[7]), Some(String::from("999")));

        assert_eq!(helpers::process_input_last_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[8]), Some(String::from("1000")));

        assert_eq!(helpers::process_input_last_output(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[9]), Some(String::from("1001")));
        
    }    
}