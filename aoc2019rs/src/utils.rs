use anyhow::Result;

use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read_input_lines(day_number: u8) -> Result<Vec<String>> {
    let file_name = format!("input{}", day_number);
    let file = BufReader::new(File::open(file_name)?);
    let result = file.lines().map(|line| line.unwrap()).collect();
    Ok(result)
}

pub fn read_input_list(day_number: u8, delimiter: u8) -> Result<Vec<String>> {
    let file_name = format!("input{}", day_number);
    let file = BufReader::new(File::open(file_name)?);
    let result = file
        .split(delimiter)
        .map(|element| String::from_utf8(element.unwrap()).unwrap())
        .collect();
    Ok(result)
}

pub fn read_input_converted_input_list<F, T>(day_number: u8, delimiter: u8, converter: F) -> Result<Vec<T>> 
where F: Fn(String) -> T
{
    let file_name = format!("input{}", day_number);
    let file = BufReader::new(File::open(file_name)?);
    let result = file
        .split(delimiter)
        .map(|element| {
            let element = element.unwrap();
            let element = String::from_utf8(element).unwrap();
            converter(element)
        })
        .collect();
    Ok(result)
}
