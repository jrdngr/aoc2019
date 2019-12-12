use anyhow::Result;

use crate::utils::input;

pub fn run() -> Result<String> {
    let _ = input::read_input_list_as::<i64>(9, b',')?;

    test_relative_base();
    
    Ok(String::from("Not implemented"))
}

fn test_relative_base() {
    use std::str::FromStr;

    let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    let result = crate::intcode::helpers::process_input(&program, &[]).into_iter()
        .flat_map(|output| i64::from_str(&output))
        .collect::<Vec<i64>>();
 
    assert_eq!(&result, &program);
}
