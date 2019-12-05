use anyhow::Result;

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

pub fn read_input_lines_as_i64(day_number: u8) -> Result<Vec<i64>> {
    let result = read_input_lines(day_number)?
        .into_iter()
        .flat_map(|line| i64::from_str(&line))
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

pub fn read_input_list_as_i64(day_number: u8, delimiter: u8) -> Result<Vec<i64>> {
    read_converted_input_list(day_number, delimiter, |element| i64::from_str(element).unwrap())
}

pub fn read_converted_input_list<F, T>(day_number: u8, delimiter: u8, converter: F) -> Result<Vec<T>> 
where F: Fn(&str) -> T
{
    let result = read_input_list(day_number, delimiter)?
        .into_iter()
        .map(|element| converter(&element))
        .collect();
    Ok(result)
}
