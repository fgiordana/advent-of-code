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
	min: usize,
	max: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PasswordEntry {
	rule: PasswordRule,
	password: String,	
}

impl FromStr for PasswordEntry {
	type Err = PasswordEntryError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		lazy_static!{
			static ref RE: Regex = Regex::new(PASSWORD_ENTRY_REGEX).unwrap();
		}
		RE.captures(s)
			.ok_or_else(||PasswordEntryError::RegexMatchError(s.to_string()))
			.and_then(|cap| Ok(Self {
				rule: PasswordRule {
					character: cap[3].parse()?,
					min: cap[1].parse()?,
					max: cap[2].parse()?,
				},
				password: cap[4].parse()?,
			}))	
	}
}


fn password_valid(entry: &PasswordEntry) -> bool {
	let count = entry.password.matches(entry.rule.character).count();
	count >= entry.rule.min && count <= entry.rule.max
}


pub fn part1() -> Result<usize> {
	let result = fs::read_to_string(Path::new(FILEPATH))?
		.split('\n')
		.map(str::parse::<PasswordEntry>)
		.map(|x| x.unwrap())
		.filter(|x| password_valid(x))
		.count();
	Ok(result)
}


#[cfg(test)]
mod tests {
	use super::*;

	const TEST_PASSWORD_ENTRY_STR: &str = "1-3 m: mmmaoou";
	const BAD_PASSWORD_ENTRY_STR: &str = "1-3 m mmmaoou"; 

	const PART1_RESULT: usize = 524;

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
					min: 1,
					max: 3,
				},
				password: "mmmaoou".to_string(),
			}
		);
		match str::parse::<PasswordEntry>(BAD_PASSWORD_ENTRY_STR) {
			Err(PasswordEntryError::RegexMatchError(_s)) => (),
			_ => panic!("Expected RegexMatchError")
		}
	}

	#[test]
	fn check_password_valid() {
		assert!(password_valid(
			&PasswordEntry {
				rule: PasswordRule {
					character: 'm',
					min: 1,
					max: 3,
				},
				password: "momomoooo".to_string(),
			}
		));
		assert!(!password_valid(
			&PasswordEntry {
				rule: PasswordRule {
					character: 'm',
					min: 1,
					max: 3,
				},
				password: "nononoooo".to_string(),
			}
		));
		assert!(!password_valid(
			&PasswordEntry {
				rule: PasswordRule {
					character: 'm',
					min: 1,
					max: 3,
				},
				password: "momomomooo".to_string(),
			}
		));
	}

	#[test]
	fn check_part1() {
		assert_eq!(part1().unwrap(), PART1_RESULT);
	}
}
