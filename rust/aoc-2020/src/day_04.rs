use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use thiserror::Error;

pub const FILEPATH: &str = "data/day_04/input.txt";

#[derive(Debug, Error)]
enum PassportParseError {
    #[error("Ivalid input string: {0}")]
    InvalidInputString(String),

    #[error("Invalid fields: {0}")]
    InvalidFields(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = PassportParseError;

    fn from_str(s: &str) -> Result<Self, PassportParseError> {
        let map: HashMap<&str, &str> = s
            .split(&[' ', ':'][..])
            .tuple_windows::<(&str, &str)>()
            .collect();
        let json = serde_json::to_string(&map)
            .map_err(|e| PassportParseError::InvalidInputString(e.to_string()))?;
        let passport: Passport = serde_json::from_str(&json)
            .map_err(|e| PassportParseError::InvalidFields(e.to_string()))?;
        Ok(passport)
    }
}

fn valid_byr(value: &str) -> bool {
    matches!(str::parse::<u32>(value), Ok(byr) if (1920..=2002).contains(&byr))
}

fn valid_iyr(value: &str) -> bool {
    matches!(str::parse::<u32>(value), Ok(iyr) if (2010..=2020).contains(&iyr))
}

fn valid_eyr(value: &str) -> bool {
    matches!(str::parse::<u32>(value), Ok(eyr) if (2020..=2030).contains(&eyr))
}

fn valid_hgt(value: &str) -> bool {
    lazy_static! {
        static ref HGT: Regex = Regex::new(r"^(\d+)cm|(\d+)in$").unwrap();
    }
    if let Some(caps) = HGT.captures(value) {
        if let Some(cms) = caps.get(1) {
            return (150..=193).contains(&str::parse::<u32>(cms.as_str()).unwrap());
        } else if let Some(inches) = caps.get(2) {
            return (59..=76).contains(&str::parse::<u32>(inches.as_str()).unwrap());
        } else {
            return false;
        }
    }
    false
}

fn valid_hcl(value: &str) -> bool {
    lazy_static! {
        static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    HCL.is_match(value)
}

fn valid_ecl(value: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&x| x == value)
}

fn valid_pid(value: &str) -> bool {
    lazy_static! {
        static ref PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    PID.is_match(value)
}

impl Passport {
    pub fn is_valid(&self) -> bool {
        valid_byr(&self.byr)
            && valid_iyr(&self.iyr)
            && valid_eyr(&self.eyr)
            && valid_hgt(&self.hgt)
            && valid_hcl(&self.hcl)
            && valid_ecl(&self.ecl)
            && valid_pid(&self.pid)
    }
}

pub fn part1() -> Result<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\n\t]").unwrap();
    }

    let content = fs::read_to_string(FILEPATH)?;
    let valid_passports = content
        .split("\n\n")
        .map(|s| str::parse(&RE.replace_all(s, " ")))
        .filter(Result::is_ok)
        .collect::<Result<Vec<Passport>, _>>()?;
    Ok(valid_passports.len())
}

pub fn part2() -> Result<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[\n\t]").unwrap();
    }

    let content = fs::read_to_string(FILEPATH)?;
    Ok(content
        .split("\n\n")
        .map(|s| str::parse(&RE.replace_all(s, " ")))
        .filter(Result::is_ok)
        .collect::<Result<Vec<Passport>, _>>()?
        .into_iter()
        .filter(Passport::is_valid)
        .count())
}

#[cfg(test)]
mod test {

    use super::*;

    const PART1_RESULT: usize = 202;
    const PART2_RESULT: usize = 137;

    #[test]
    fn test_passport_deserialize() {
        let s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        let passport = str::parse::<Passport>(s).unwrap();
        assert_eq!(
            passport,
            Passport {
                ecl: "gry".to_string(),
                pid: "860033327".to_string(),
                eyr: "2020".to_string(),
                hcl: "#fffffd".to_string(),
                byr: "1937".to_string(),
                iyr: "2017".to_string(),
                cid: Some("147".to_string()),
                hgt: "183cm".to_string(),
            }
        );

        let s = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147";
        match str::parse::<Passport>(s) {
            Err(PassportParseError::InvalidFields(_)) => (),
            _ => panic!("Expected a PassportParseError::InvalidFields error"),
        }
    }

    #[test]
    fn check_part1() {
        let result = part1().unwrap();
        assert_eq!(result, PART1_RESULT);
    }

	#[test]
	fn test_valid_passport() {
		assert!(Passport {
			ecl: "grn".to_string(),
            pid: "087499704".to_string(),
            eyr: "2030".to_string(),
            hcl: "#623a2f".to_string(),
            byr: "1980".to_string(),
            iyr: "2012".to_string(),
			cid: None,
            hgt: "74in".to_string(),
		}.is_valid());
		assert!(Passport {
			ecl: "blu".to_string(),
            pid: "896056539".to_string(),
            eyr: "2029".to_string(),
            hcl: "#a97842".to_string(),
            byr: "1989".to_string(),
            iyr: "2014".to_string(),
			cid: Some("129".to_string()),
            hgt: "165cm".to_string(),
		}.is_valid());
		assert!(Passport {
			ecl: "hzl".to_string(),
            pid: "545766238".to_string(),
            eyr: "2022".to_string(),
            hcl: "#888785".to_string(),
            byr: "2001".to_string(),
            iyr: "2015".to_string(),
			cid: Some("88".to_string()),
            hgt: "164cm".to_string(),
		}.is_valid());
		assert!(Passport {
			ecl: "blu".to_string(),
            pid: "093154719".to_string(),
            eyr: "2021".to_string(),
            hcl: "#b6652a".to_string(),
            byr: "1944".to_string(),
            iyr: "2010".to_string(),
			cid: Some("88".to_string()),
            hgt: "158cm".to_string(),
		}.is_valid());
		assert!(!Passport {
			ecl: "amb".to_string(),
            pid: "186cm".to_string(),
            eyr: "1972".to_string(),
            hcl: "#18171d".to_string(),
            byr: "1926".to_string(),
            iyr: "2018".to_string(),
			cid: Some("100".to_string()),
            hgt: "170".to_string(),
		}.is_valid());
		assert!(!Passport {
			ecl: "grn".to_string(),
            pid: "012533040".to_string(),
            eyr: "1967".to_string(),
            hcl: "#602927".to_string(),
            byr: "1946".to_string(),
            iyr: "2019".to_string(),
			cid: None, 
            hgt: "170cm".to_string(),
		}.is_valid());
		assert!(!Passport {
			ecl: "brn".to_string(),
            pid: "021572410".to_string(),
            eyr: "2020".to_string(),
            hcl: "dab227".to_string(),
            byr: "1992".to_string(),
            iyr: "2012".to_string(),
			cid: Some("277".to_string()),
            hgt: "182cm".to_string(),
		}.is_valid());
		assert!(!Passport {
			ecl: "zzz".to_string(),
            pid: "3556412378".to_string(),
            eyr: "2038".to_string(),
            hcl: "74454a".to_string(),
            byr: "2007".to_string(),
            iyr: "2023".to_string(),
			cid: None,
            hgt: "59cm".to_string(),
		}.is_valid());
		assert!(!Passport {
			ecl: "blu".to_string(),
            pid: "0093154719".to_string(),
            eyr: "2021".to_string(),
            hcl: "#b6652a".to_string(),
            byr: "1944".to_string(),
            iyr: "2010".to_string(),
			cid: Some("88".to_string()),
            hgt: "158cm".to_string(),
		}.is_valid());
	}

    #[test]
    fn check_part2() {
        let result = part2().unwrap();
        assert_eq!(result, PART2_RESULT);
    }
}
