use std::str::FromStr;

use crate::intcode::{IntcodeMachine, IntcodeBlockingInput, IntcodeHistoryOutput};

pub struct IntcodeDebugger {
    machine: IntcodeMachine<IntcodeBlockingInput, IntcodeHistoryOutput>,
}

impl IntcodeDebugger {
    pub fn new(machine_code: &[i64]) -> Self {
        Self {
            machine: IntcodeMachine::new_blocking_machine(machine_code),
        }
    }

    pub fn start_debugging(&mut self) {

    }

    fn step(&mut self) {
        let input = crate::utils::input::read_input_with_prompt("").unwrap();
        
        match input.as_ref() {
            "q" => panic!("Aborting execution"),
            _ => println!("Unknown debugger command: {}", input),
        }
    }
}

enum DebuggerCommand {
    Step,
    State,
    Memory,
    Abort,
    Skip(usize),
    MemoryRange(usize, usize),
    Unknown,
}

impl FromStr for DebuggerCommand {
    type Err = anyhow::Error;
    
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse(input) {
            Ok((_, command)) => Ok(command),
            Err(_) => Ok(DebuggerCommand::Unknown),
        }
    }
}

fn parse(input: &str) -> nom::IResult<&str, DebuggerCommand> {
    use nom::{
        character::complete::{
            digit1,
            line_ending,
        },
        branch::alt,
        bytes::complete::tag,
        combinator::map,
        sequence::separated_pair,
    };

    alt((
        map(separated_pair(tag("skip"), tag(" "), digit1), |_| DebuggerCommand::Unknown),
        map(separated_pair(tag("mem"), tag(" "), 
            separated_pair(digit1, tag(","), digit1)), |_| DebuggerCommand::Unknown),
        map(line_ending, |_| DebuggerCommand::Step),
        map(tag("state"), |_| DebuggerCommand::State),
        map(tag("mem"), |_| DebuggerCommand::Memory),
        map(tag("q"), |_| DebuggerCommand::Abort),
    ))(input)
}
