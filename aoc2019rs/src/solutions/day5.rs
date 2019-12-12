use anyhow::Result;

use crate::utils::input;
use crate::intcode::helpers;

// Part 1: 9025675
// Part 2: 11981754

pub fn run() -> Result<String> {
    let program = input::read_input_list_as::<i64>(5, b',')?;

    let part1 = helpers::process_input(&program, &[1])
        .last()
        .unwrap()
        .clone();
    let part2 = helpers::process_input(&program, &[5])
        .last()
        .unwrap()
        .clone();

    Ok(format!("Part 1: {}\nPart 2: {}", part1, part2))
}

#[cfg(test)]
mod tests {
    use crate::intcode::helpers;

    #[test]
    fn day5_part1() {
        let program = day5_input();
        let result = helpers::process_input(&program, &[1]).last().unwrap().clone();
        assert_eq!(result, "9025675");
    }

    fn day5_input() -> Vec<i64> {
        crate::utils::input::read_input_list_as::<i64>(5, b',').unwrap()
    }

    #[test]
    fn day5_comparison_tests() {
        assert_eq!(helpers::process_input(&[3,9,8,9,10,9,4,9,99,-1,8], &[7]).last().unwrap().clone(), "0");
        assert_eq!(helpers::process_input(&[3,9,8,9,10,9,4,9,99,-1,8], &[8]).last().unwrap().clone(), "1");
        
        assert_eq!(helpers::process_input(&[3,9,7,9,10,9,4,9,99,-1,8], &[7]).last().unwrap().clone(), "1");
        assert_eq!(helpers::process_input(&[3,9,7,9,10,9,4,9,99,-1,8], &[9]).last().unwrap().clone(), "0");

        assert_eq!(helpers::process_input(&[3,3,1108,-1,8,3,4,3,99], &[7]).last().unwrap().clone(), "0");
        assert_eq!(helpers::process_input(&[3,3,1108,-1,8,3,4,3,99], &[8]).last().unwrap().clone(), "1");

        assert_eq!(helpers::process_input(&[3,3,1107,-1,8,3,4,3,99], &[7]).last().unwrap().clone(), "1");
        assert_eq!(helpers::process_input(&[3,3,1107,-1,8,3,4,3,99], &[9]).last().unwrap().clone(), "0");
    }

    #[test]
    fn day5_jump_position_tests() {
        assert_eq!(helpers::process_input(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0]).last().unwrap().clone(), "0");
        assert_eq!(helpers::process_input(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[1]).last().unwrap().clone(), "1");
    }

    #[test]
    fn day5_jump_immediate_tests() {
        assert_eq!(helpers::process_input(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0]).last().unwrap().clone(), "0");
        assert_eq!(helpers::process_input(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[1]).last().unwrap().clone(), "1");
    } 

    #[test]
    fn day5_complex_test() {
        assert_eq!(helpers::process_input(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[7]).last().unwrap().clone(), 
            "999");

        assert_eq!(helpers::process_input(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[8]).last().unwrap().clone(), 
            "1000");

        assert_eq!(helpers::process_input(
            &[3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
              1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
              999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99], &[9]).last().unwrap().clone(), 
            "1001");
        
    }    
}