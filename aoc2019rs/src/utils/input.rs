use anyhow::Result;
use rustyline::Editor;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

pub fn input_file_name(day_number: u8) -> String {
    format!("input/input{}", day_number)
}

pub fn input_file_reader(day_number: u8) -> Result<BufReader<File>> {
    let file = File::open(input_file_name(day_number))?;
    Ok(BufReader::new(file))
}

pub fn read_input_lines(day_number: u8) -> Result<Vec<String>> {
    let file = input_file_reader(day_number)?;
    let result = file.lines().flatten().collect();
    Ok(result)
}

pub fn read_input_lines_as<T>(day_number: u8) -> Result<Vec<T>> 
where T: FromStr
{
    let result = read_input_lines(day_number)?
        .into_iter()
        .flat_map(|line| T::from_str(&line))
        .collect();
    Ok(result)
}

pub fn read_input_list(day_number: u8, delimiter: u8) -> Result<Vec<String>> {
    let file = input_file_reader(day_number)?;
    let result = file
        .split(delimiter)
        .flatten()
        .flat_map(String::from_utf8)
        .collect();
    Ok(result)
}

pub fn read_input_list_as<T>(day_number: u8, delimiter: u8) -> Result<Vec<T>> 
where T: FromStr
{
    let result = read_input_list(day_number, delimiter)?
        .into_iter()
        .flat_map(|element| T::from_str(&element))
        .collect();
    Ok(result)
}


pub fn read_input() -> Result<String> {
    let mut rl = Editor::<()>::new();
    let readline = rl.readline("Enter input: ");
    Ok(readline?)
}
