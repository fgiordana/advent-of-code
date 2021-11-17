use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

pub const FILEPATH: &str = "data/day_02/input.txt";
pub const PASSWORD_ENTRY_REGEX: &str = r"(\d+)-(\d+) (.): (.+)";

#[derive(Error, Debug)]
pub enum PasswordEntryError {
    #[error("Couldn't match regex to string: {0}")]
    RegexMatchError(String),

    #[error("Parse error: {0}")]
    ParseCharError(#[from] std::char::ParseCharError),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Infallible: {0}")]
    Infallible(#[from] std::convert::Infallible),
}

#[derive(Clone, Debug, PartialEq)]
pub struct PasswordRule {
    character: char,
    first: usize,
    second: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PasswordEntry {
    rule: PasswordRule,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = PasswordEntryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(PASSWORD_ENTRY_REGEX).unwrap();
        }
        RE.captures(s)
            .ok_or_else(|| PasswordEntryError::RegexMatchError(s.to_string()))
            .and_then(|cap| {
                Ok(Self {
                    rule: PasswordRule {
                        character: cap[3].parse()?,
                        first: cap[1].parse()?,
                        second: cap[2].parse()?,
                    },
                    password: cap[4].parse()?,
                })
            })
    }
}

fn password_valid_part1(entry: &PasswordEntry) -> bool {
    let count = entry.password.matches(entry.rule.character).count();
    count >= entry.rule.first && count <= entry.rule.second
}

fn password_valid_part2(entry: &PasswordEntry) -> bool {
    let mut matches = 0;
    if entry.password.len() >= entry.rule.first
        && entry.password.as_bytes()[entry.rule.first - 1] as char == entry.rule.character
    {
        matches += 1;
    };
    if entry.password.len() >= entry.rule.second
        && entry.password.as_bytes()[entry.rule.second - 1] as char == entry.rule.character
    {
        matches += 1;
    };
    matches == 1
}

pub fn part1() -> Result<usize> {
    let result = fs::read_to_string(Path::new(FILEPATH))?
        .split('\n')
        .map(str::parse::<PasswordEntry>)
        .map(|x| x.unwrap())
        .filter(|x| password_valid_part1(x))
        .count();
    Ok(result)
}

pub fn part2() -> Result<usize> {
    let result = fs::read_to_string(Path::new(FILEPATH))?
        .split('\n')
        .map(str::parse::<PasswordEntry>)
        .map(|x| x.unwrap())
        .filter(|x| password_valid_part2(x))
        .count();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PASSWORD_ENTRY_STR: &str = "1-3 m: mmmaoou";
    const BAD_PASSWORD_ENTRY_STR: &str = "1-3 m mmmaoou";

    const PART1_RESULT: usize = 524;
    const PART2_RESULT: usize = 485;

    #[test]
    fn input_file_exists() {
        assert!(Path::new(FILEPATH).exists());
    }

    #[test]
    fn correct_regex() {
        let re = Regex::new(PASSWORD_ENTRY_REGEX).unwrap();
        assert!(re.captures(TEST_PASSWORD_ENTRY_STR).is_some());
        assert!(re.captures(BAD_PASSWORD_ENTRY_STR).is_none());
    }

    #[test]
    fn correct_parsing() {
        assert_eq!(
            str::parse::<PasswordEntry>(TEST_PASSWORD_ENTRY_STR).unwrap(),
            PasswordEntry {
                rule: PasswordRule {
                    character: 'm',
                    first: 1,
                    second: 3,
                },
                password: "mmmaoou".to_string(),
            }
        );
        match str::parse::<PasswordEntry>(BAD_PASSWORD_ENTRY_STR) {
            Err(PasswordEntryError::RegexMatchError(_s)) => (),
            _ => panic!("Expected RegexMatchError"),
        }
    }

    #[test]
    fn check_password_valid_part1() {
        assert!(password_valid_part1(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "momomoooo".to_string(),
        }));
        assert!(!password_valid_part1(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "nononoooo".to_string(),
        }));
        assert!(!password_valid_part1(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "momomomooo".to_string(),
        }));
    }

    #[test]
    fn check_password_valid_part2() {
        assert!(password_valid_part2(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "moooooo".to_string(),
        }));
        assert!(password_valid_part2(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "oomoooo".to_string(),
        }));
        assert!(!password_valid_part2(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "momooooo".to_string(),
        }));
        assert!(!password_valid_part2(&PasswordEntry {
            rule: PasswordRule {
                character: 'm',
                first: 1,
                second: 3,
            },
            password: "oooooo".to_string(),
        }));
    }

    #[test]
    fn check_part1() {
        assert_eq!(part1().unwrap(), PART1_RESULT);
    }

    #[test]
    fn check_part2() {
        assert_eq!(part2().unwrap(), PART2_RESULT);
    }
}
