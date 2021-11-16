use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs;

pub const FILEPATH: &str = "data/day_01/input.txt";
pub const TARGET_SUM: i64 = 2020;

pub fn day_01() -> Result<(i64, i64)> {
    fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_combinations()
        .find(|(x, y)| x + y == TARGET_SUM)
        .with_context(|| format!("No two elements sum up to {}", TARGET_SUM))
}
