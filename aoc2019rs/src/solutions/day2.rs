use anyhow::Result;

use crate::utils;

pub fn run() -> Result<String> {
    let input = utils::read_input_list_as_i64(2, b',')?;
    dbg!(input);

    Ok(String::from(""))
}