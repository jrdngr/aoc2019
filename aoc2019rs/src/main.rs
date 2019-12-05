use anyhow::{anyhow, bail, Result};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<()> {
    let day_num = std::env::args().nth(1).ok_or(anyhow!("Please provide a day number as the first argument"))?;
    
    match day_num.as_ref() {
        "1" => (),
        "2" => (),
        "3" => (),
        "4" => (),
        "5" => (),
        _   => bail!("Invalid day number: {}", day_num),
    }

    Ok(())
}
