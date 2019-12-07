use anyhow::{bail, Result};

use std::str::FromStr;
use std::collections::HashSet;
use std::ops::Add;

use crate::utils::input;

// Part 1: 266
// Part 2: 

pub fn run() -> Result<String> {
    let input = input::read_input_lines(3)?;
    let wire1: Vec<WireSegment> = input[0].split(',').flat_map(WireSegment::from_str).collect();
    let wire2: Vec<WireSegment> = input[1].split(',').flat_map(WireSegment::from_str).collect();

    let result = solve(wire1, wire2);
    
    Ok(format!("{}", result))
}

fn solve(wire1: Vec<WireSegment>, wire2: Vec<WireSegment>) -> i64 {
    let mut wire_grid1 = HashSet::<(i64, i64)>::new();
    add_wire(&mut wire_grid1, wire1);

    let mut wire_grid2 = HashSet::<(i64, i64)>::new();
    add_wire(&mut wire_grid2, wire2);

    let closest_cross = wire_grid1.into_iter()
        .filter(|cell| cell != &(0, 0))
        .filter(|cell| wire_grid2.contains(cell))
        .min_by_key(distance)
        .unwrap();

    distance(&closest_cross)
}

fn distance(cell: &(i64, i64)) -> i64 {
    cell.0.abs() + cell.1.abs()
}

fn add_wire(grid: &mut HashSet<(i64, i64)>, wire: Vec<WireSegment>) {
    let mut current_position = (0, 0);
    for segment in wire {
        current_position = add_segment(grid, current_position, segment);
    }
}

fn add_segment(grid: &mut HashSet<(i64, i64)>, start: (i64, i64), segment: WireSegment) -> (i64, i64) {
    let expanded = expand_segment(start, segment);
    
    for cell in &expanded {
        grid.insert(*cell);
    }

    *expanded.last().unwrap()
}

fn expand_segment(start: (i64, i64), segment: WireSegment) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    let target = start + segment;

    match segment {
        WireSegment::Up(_) => {
            for y in start.1..=target.1 {
                result.push((start.0, y));
            }
        },
        WireSegment::Down(_) => {
            for y in (target.1..=start.1).rev() {
                result.push((start.0, y));
            }
        },
        WireSegment::Right(_) => {
            for x in start.0..=target.0 {
                result.push((x, start.1))
            }
        },
        WireSegment::Left(_) => {
            for x in (target.0..=start.0).rev() {
                result.push((x, start.1))
            }
        },
    }

    result
}

#[derive(Debug, Clone, Copy)]
enum WireSegment {
    Up(i64),
    Down(i64),
    Right(i64),
    Left(i64),
}

impl FromStr for WireSegment {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let split = input.split_at(1);
        let distance = i64::from_str(split.1)?;
        
        Ok(match split.0 {
            "U" => WireSegment::Up(distance),
            "D" => WireSegment::Down(distance),
            "R" => WireSegment::Right(distance),
            "L" => WireSegment::Left(distance),
            _   => bail!("Invalid direction:{}", split.0)
        })
    }
}

impl Add<WireSegment> for (i64, i64) {
    type Output = (i64, i64);

    fn add(self, other: WireSegment) -> Self::Output {
        match other {
            WireSegment::Up(distance) => (self.0, self.1 + distance),
            WireSegment::Down(distance) => (self.0, self.1 - distance),
            WireSegment::Right(distance) => (self.0 + distance, self.1),
            WireSegment::Left(distance) => (self.0 - distance, self.1),
        }
    }
}
