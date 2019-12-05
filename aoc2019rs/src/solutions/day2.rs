use anyhow::Result;

use crate::utils;

pub fn run() -> Result<String> {
    let input = utils::read_input_list_as::<usize>(2, b',')?;

    Ok(String::from(""))
}
