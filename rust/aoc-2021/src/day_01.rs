use anyhow::Result;
use itertools::Itertools;
use std::fs;

const FILEPATH: &str = "data/day_01/input.txt";

pub fn part1() -> Result<usize> {
    Ok(fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count())
}

pub fn part2() -> Result<usize> {
    Ok(fs::read_to_string(FILEPATH)?
        .split('\n')
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .tuple_windows()
        .map(|(x, y, z)| x + y + z)
        .collect::<Vec<_>>()
        .into_iter()
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count())
}

#[cfg(test)]
mod test {
    use super::*;

    const PART1_RESULT: usize = 1393;
    const PART2_RESULT: usize = 1359;

    #[test]
    fn chck_part1() {
        let result = part1().unwrap();
        assert_eq!(result, PART1_RESULT);
    }

    #[test]
    fn check_part2() {
        let result = part2().unwrap();
        assert_eq!(result, PART2_RESULT);
    }
}
