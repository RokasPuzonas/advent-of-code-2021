use std::num::ParseIntError;

#[derive(Debug)]
pub enum ParseCommandError {
    ParseEnumError,
    ParseIntError(ParseIntError),
}

pub enum CommandType {
    Forward,
    Down,
    Up
}

pub struct Command(CommandType, u32);

fn parse_line(line: &str) -> Result<Command, ParseCommandError> {
    let parts: Vec<&str> = line.split(' ').collect();
    let command = match parts[0] {
        "up" => Ok(CommandType::Up),
        "down" => Ok(CommandType::Down),
        "forward" => Ok(CommandType::Forward),
        _ => Err(ParseCommandError::ParseEnumError)
    }?;
    let amount = parts[1].parse().map_err(ParseCommandError::ParseIntError)?;
    Ok(Command(command, amount))
}

pub fn parse_input(input: &str) -> Result<Vec<Command>, ParseCommandError> {
    input.split_terminator('\n')
        .map(parse_line)
        .collect()
}

pub fn part1(commands: &[Command]) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;
    for command in commands {
        match command.0 {
            CommandType::Up      => depth -= command.1,
            CommandType::Down    => depth += command.1,
            CommandType::Forward => horizontal += command.1,
        }
    }
    return depth * horizontal;
}

pub fn part2(commands: &[Command]) -> u32 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for command in commands {
        match command.0 {
            CommandType::Up      => aim -= command.1,
            CommandType::Down    => aim += command.1,
            CommandType::Forward => {
                horizontal += command.1;
                    depth += aim * command.1;
            }
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
            Command(CommandType::Forward, 5),
            Command(CommandType::Down, 5),
            Command(CommandType::Forward, 8),
            Command(CommandType::Up, 3),
            Command(CommandType::Down, 8),
            Command(CommandType::Forward, 2)
        ];
        let result = part1(&commands);
        assert_eq!(result, 150);
    }

    #[test]
    fn part2_example() {
        let commands = [
            Command(CommandType::Forward, 5),
            Command(CommandType::Down, 5),
            Command(CommandType::Forward, 8),
            Command(CommandType::Up, 3),
            Command(CommandType::Down, 8),
            Command(CommandType::Forward, 2)
        ];
        let result = part2(&commands);
        assert_eq!(result, 900)
    }
}

