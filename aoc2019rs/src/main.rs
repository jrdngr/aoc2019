use anyhow::{anyhow, bail, Result};

mod solutions;
mod utils;

fn main() -> Result<()> {
    let day_num = std::env::args().nth(1).ok_or(anyhow!("Please provide a day number as the first argument"))?;
    
    println!("Running day #{}...", day_num);

    let result = match day_num.as_ref() {
        "1" => solutions::day1::run()?,
        "2" => solutions::day2::run()?,
        "3" => solutions::day3::run()?,
        "4" => solutions::day4::run()?,
        "5" => solutions::day5::run()?,
        _   => bail!("Invalid day number: {}", day_num),
    };

    println!("{}", result);

    Ok(())
}
