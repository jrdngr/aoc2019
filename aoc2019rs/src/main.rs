use anyhow::{anyhow, bail, Result};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<()> {
    let day_num = std::env::args().nth(1).ok_or(anyhow!("Please provide a day number as the first argument"))?;
    
    println!("Running day #{}...", day_num);

    let result = match day_num.as_ref() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        _   => bail!("Invalid day number: {}", day_num),
    };

    println!("{}", result);

    Ok(())
}
