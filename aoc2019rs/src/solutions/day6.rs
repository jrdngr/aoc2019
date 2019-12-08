use anyhow::Result;
use nom::IResult;

use std::collections::HashMap;

use crate::utils::input;

type OrbitMap<'a> = HashMap<&'a str, &'a str>;

// Part 1: 224901
// Part 2: 

pub fn run() -> Result<String> {
    let input = input::read_input_lines(6)?;
    let orbits = parse_orbit_list(&input);
    let total_orbits = total_orbits(&orbits);

    let distance_to_san = distance_between("YOU", "SAN", &orbits);
    
    Ok(format!("{}", total_orbits))
}

fn distance_between(first: &str, second: &str, orbits: &OrbitMap) {
    let you_distance = distance_from_com(first, orbits);
    let san_distance = distance_from_com(second, orbits);


}

fn find_least_common_ancestor<'a>(first: &'a str, second: &'a str, orbits: &OrbitMap) -> &'a str {
    ""
}

fn total_orbits(orbits: &OrbitMap) -> usize {
    orbits.iter()
        .map(|(object, _)| distance_from_com(object, orbits))
        .sum()
}

fn distance_from_com(object: &str, orbits: &OrbitMap) -> usize {
    let parent = orbits.get(object).expect(&format!("Object {} not found", object));
    if *parent == "COM" {
        1
    } else {
        1 + distance_from_com(parent, orbits)
    }
}

fn parse_orbit_list(orbits: &[String]) -> OrbitMap {
    orbits.into_iter()
          .flat_map(|orbit| parse_orbit(&orbit))
          .map(|(_, parsed_orbit)| (parsed_orbit.1, parsed_orbit.0))
          .collect()
}

fn parse_orbit(orbit_string: &str) -> IResult<&str, (&str, &str)> {
    use nom::{
        character::complete::alphanumeric1,
        bytes::complete::tag,
        sequence::separated_pair,
    };

    separated_pair(alphanumeric1, tag(")"), alphanumeric1)(orbit_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_orbit("COM)B").unwrap().1, ("COM", "B"));
        assert_eq!(parse_orbit("B)C").unwrap().1, ("B", "C"));
    }

    #[test]
    fn test_parse_list() {
        let input = example();
        let orbits = parse_orbit_list(&input);
        
        assert!(orbits["B"] == "COM");
        assert!(orbits["C"] == "B");
        assert!(orbits["D"] == "C");
    }

    #[test]
    fn test_distance() {
        let input = example();
        let orbits = parse_orbit_list(&input);

        assert_eq!(distance_from_com("B", &orbits), 1);
        assert_eq!(distance_from_com("C", &orbits), 2);
        assert_eq!(distance_from_com("D", &orbits), 3);
        assert_eq!(distance_from_com("L", &orbits), 7);
    }

    #[test]
    fn test_example() {
        let input = example();
        let orbits = parse_orbit_list(&input);
        assert_eq!(total_orbits(&orbits), 42);
    }

    fn example() -> Vec<String> {
        ["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"]
            .into_iter()
            .map(|s| String::from(*s))
            .collect()
    }
}
