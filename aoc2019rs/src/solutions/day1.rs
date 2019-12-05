use anyhow::Result;


use crate::utils;

pub fn run() -> Result<String> {
    let input = utils::read_input_lines_as_i64(1)?;

    let result = input.into_iter()
        .map(fuel_by_weight)
        .fold(0, total_fuel);
    
    Ok(format!("{}", result))
}

fn fuel_by_weight(weight: i64) -> i64 {
    (weight / 3) - 2
}

fn total_fuel(current_total: i64, weight: i64) -> i64 {
    if weight <= 0 {
        current_total
    } else {
        total_fuel(current_total + weight, fuel_by_weight(weight))
    }
}
