use anyhow::Result;
use nom::{
    IResult,
    character::complete::alpha1,
    bytes::complete::tag,
    sequence::tuple,
};

pub fn run() -> Result<String> {
    Ok(String::from("Day 6"))
}

fn parse_orbit(orbit_string: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((alpha1, tag(")"), alpha1))(orbit_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_orbit("COM)B").unwrap().1, ("COM", ")", "B"));
        assert_eq!(parse_orbit("B)C").unwrap().1, ("B", ")", "C"));
    }
}