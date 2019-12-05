use anyhow::Result;

// Part 1: 511
// Part 2: 

pub fn run() -> Result<String> {
    let range: std::ops::RangeInclusive<usize> = 359282..=820401;

    let result = range
        .filter(digits_increasing)
        .filter(contains_double)
        .count();

    Ok(format!("{}", result))
}

fn digits_increasing(value: &usize) -> bool {
    let digits = into_digits(value);

    for i in 0..digits.len()-1 {
        if digits[i] > digits[i+1] {
            return false;
        }
    }

    true
}

fn contains_double(value: &usize) -> bool {
    let digits = into_digits(value);

    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            return true;
        }
    }

    false
}

fn into_digits(value: &usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(6);

    let mut current_value = *value;
    while current_value > 0 {
        result.push(current_value % 10);
        current_value /= 10;
    }

    result.into_iter().rev().collect()
}
