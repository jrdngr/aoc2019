use anyhow::{bail, Result};

use std::str::FromStr;

use crate::utils;

pub fn run() -> Result<String> {
    let input = utils::read_input_lines(3)?;
    let wire1: Vec<WireSegment> = input[0].split(',').flat_map(WireSegment::from_str).collect();
    let wire2: Vec<WireSegment> = input[1].split(',').flat_map(WireSegment::from_str).collect();

    dbg!(wire1);

    Ok(String::from(""))
}

#[derive(Debug)]
enum WireSegment {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl FromStr for WireSegment {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let split = input.split_at(1);
        let distance = usize::from_str(split.1)?;
        
        Ok(match split.0 {
            "U" => WireSegment::Up(distance),
            "D" => WireSegment::Down(distance),
            "L" => WireSegment::Left(distance),
            "R" => WireSegment::Right(distance),
            _   => bail!("Invalid direction:{}", split.0)
        })
    }
}
