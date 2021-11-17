use anyhow::{Context, Result};
use itertools::Itertools;
use log::info;
use std::fs;

const FILEPATH: &str = "data/day_01/input.txt";
const TARGET_SUM: i64 = 2020;

pub fn part1() -> Result<i64> {
    let (x, y) = fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(x, y)| x + y == TARGET_SUM)
        .with_context(|| format!("No two elements sum up to {}", TARGET_SUM))?;

    info!("x: {}, y: {}", x, y);
    info!("x + y = {}", x + y);
    info!("x * y = {}", x * y);

    Ok(x * y)
}

pub fn part2() -> Result<i64> {
    let (x, y, z) = fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(x, y, z)| x + y + z == TARGET_SUM)
        .with_context(|| format!("No three elements sum up to {}", TARGET_SUM))?;

    info!("x: {}, y: {}, z: {}", x, y, z);
    info!("x + y + z = {}", x + y + z);
    info!("x * y + z = {}", x * y * z);

    Ok(x * y * z)
}


#[cfg(test)]
mod tests {
	use std::path::Path;

	use super::*;

	const PART1_RESULT: i64 = 55776;
	const PART2_RESULT: i64 = 223162626;


	#[test]
	fn input_file_exists() {
    	assert!(Path::new(FILEPATH).exists());
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
