pub fn i64_into_digits(value: &i64) -> Vec<usize> {
    usize_into_digits(&(*value as usize))
}

pub fn usize_into_digits(value: &usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(6);

    let mut current_value = *value;
    while current_value > 0 {
        result.push(current_value % 10);
        current_value = current_value / 10;
    }

    result.into_iter().rev().collect()
}