use anyhow::Result;
use nom::IResult;

use crate::utils::input;
use crate::utils::graph::Graph;

// Part 1: 224901
// Part 2: 

pub fn run() -> Result<String> {
    let input = input::read_input_lines(6)?;
    let orbits = parse_orbit_list(&input);
    let total_orbits = total_orbits(&orbits);

    let distance_to_san = distance_between("YOU", "SAN", &orbits);
    
    Ok(format!("{}", total_orbits))
}

fn distance_between(first: &str, second: &str, orbits: &Graph<&str>) {
    let you_distance = distance_from_com(first, orbits);
    let san_distance = distance_from_com(second, orbits);


}

fn find_least_common_ancestor<'a>(first: &'a str, second: &'a str, orbits: &Graph<&str>) -> &'a str {
    ""
}

fn total_orbits(orbits: &Graph<&str>) -> usize {
    orbits.nodes().iter()
        .map(|object| distance_from_com(object, orbits))
        .sum()
}

fn distance_from_com(object: &str, orbits: &Graph<&str>) -> usize {
    orbits.path_bfs(&object, &"COM").expect("Error").len() - 1
}

fn parse_orbit_list(orbits: &[String]) -> Graph<&str> {
    let mut graph = Graph::new();

    orbits.into_iter()
            .flat_map(|orbit| parse_orbit(&orbit))
            .map(|(_, parsed_orbit)| (parsed_orbit.1, parsed_orbit.0))
            .for_each(|(from, to)| {
                graph.add_node(from);
                graph.add_node(to);
                graph.add_edge(&from, &to);
            });

    graph
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
