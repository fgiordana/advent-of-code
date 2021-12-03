use anyhow::Result;
use itertools::Itertools;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

const FILEPATH: &str = "data/day_02/input.txt";

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CommandError {
    #[error("Cannot parse command {0}")]
    ParseCommandError(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Command {
    Forward(u64),
    Up(u64),
    Down(u64),
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .tuple_windows()
            .next()
            .map(|(x, y)| {
                let n = str::parse::<u64>(y)
                    .map_err(|_| CommandError::ParseCommandError(s.to_string()))?;
                match x {
                    "forward" => Ok(Command::Forward(n)),
                    "up" => Ok(Command::Up(n)),
                    "down" => Ok(Command::Down(n)),
                    _ => Err(CommandError::ParseCommandError(s.to_string())),
                }
            })
            .unwrap_or_else(|| Err(CommandError::ParseCommandError(s.to_string())))
    }
}

pub fn part1() -> Result<u64> {
    let (h, d) = fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<Command>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .fold((0u64, 0u64), |(h, d), cmd| match cmd {
            Command::Forward(x) => (h + x, d),
            Command::Up(x) => (h, u64::checked_sub(d, x).unwrap()),
            Command::Down(x) => (h, d + x),
        });
    Ok(h * d)
}

pub fn part2() -> Result<u64> {
    let (h, d, _aim) = fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<Command>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .fold((0u64, 0u64, 0i64), |(h, d, aim), cmd| match cmd {
            Command::Forward(x) => (h + x, (d as i64 + aim * x as i64).try_into().unwrap(), aim),
            Command::Up(x) => (h, d, aim - x as i64),
            Command::Down(x) => (h, d, aim + x as i64),
        });
    Ok(h * d)
}

#[cfg(test)]
mod test {

    use super::*;

    const PART1_RESULT: u64 = 1938402;
    const PART2_RESULT: u64 = 1947878632;

    #[test]
    fn test_command_parsing() {
        assert_eq!(str::parse::<Command>("forward 5"), Ok(Command::Forward(5)));
        assert_eq!(
            str::parse::<Command>("forward 5 abc"),
            Ok(Command::Forward(5))
        );
        assert_eq!(
            str::parse::<Command>("forward"),
            Err(CommandError::ParseCommandError("forward".to_string()))
        );
        assert_eq!(
            str::parse::<Command>("for 5"),
            Err(CommandError::ParseCommandError("for 5".to_string()))
        );
    }

    #[test]
    fn check_part1() {
        let result = part1().unwrap();
        assert_eq!(result, PART1_RESULT);
    }

    #[test]
    fn check_part2() {
        let result = part2().unwrap();
        assert_eq!(result, PART2_RESULT);
    }
}
