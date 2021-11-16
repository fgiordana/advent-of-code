use anyhow::{Context, Result};
use itertools::Itertools;
use log::info;
use std::fs;

pub const FILEPATH: &str = "data/day_01/input.txt";
pub const TARGET_SUM: i64 = 2020;

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
