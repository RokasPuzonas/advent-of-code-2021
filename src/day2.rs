use std::fs::File;
use std::io::{prelude::*, self};
use std::num::ParseIntError;

#[derive(Debug)]
pub enum InputFromFileError {
    ParseCommandError,
    ParseIntError(ParseIntError),
    IoError(io::Error),
}

pub enum Command {
    Forward,
    Down,
    Up
}

pub struct CommandLine(Command, u32);

fn parse_line(line: &str) -> Result<CommandLine, InputFromFileError> {
    let parts: Vec<&str> = line.split(' ').collect();
    let command = match parts[0] {
        "up" => Ok(Command::Up),
        "down" => Ok(Command::Down),
        "forward" => Ok(Command::Forward),
        _ => Err(InputFromFileError::ParseCommandError)
    }?;
    let amount = parts[1].parse().map_err(InputFromFileError::ParseIntError)?;
    Ok(CommandLine(command, amount))
}

pub fn input_from_file(filename: &str) -> Result<Vec<CommandLine>, InputFromFileError> {
    let mut file = File::open(filename).map_err(InputFromFileError::IoError)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(InputFromFileError::IoError)?;

    contents.split_terminator('\n')
        .map(parse_line)
        .collect()
}

pub fn part1(commands: &[CommandLine]) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;
    for command in commands {
        match command.0 {
            Command::Up      => depth -= command.1,
            Command::Down    => depth += command.1,
            Command::Forward => horizontal += command.1,
        }
    }
    return depth * horizontal;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let commands = [
            CommandLine(Command::Forward, 5),
            CommandLine(Command::Down, 5),
            CommandLine(Command::Forward, 8),
            CommandLine(Command::Up, 3),
            CommandLine(Command::Down, 8),
            CommandLine(Command::Forward, 2)
        ];
        let result = part1(&commands);
        assert_eq!(result, 150);
    }
}

