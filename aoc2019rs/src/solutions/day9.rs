use anyhow::{bail, Result};

use crate::utils::input;

pub fn run() -> Result<String> {
    let _ = input::read_input_list_as::<i64>(9, b',')?;

    bail!("Not implemented")
}
