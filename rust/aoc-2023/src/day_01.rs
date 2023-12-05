use std::fs;
use anyhow::Result;


const FILEPATH: &str = "data/day_01/input.txt";

pub fn run() -> Result<u32> {
   let input = fs::read_to_string(FILEPATH)?;
   solve(&input) 
}

pub fn solve(input: &str) -> Result<u32> {

    let result = input 
        .split('\n')
        .map(|line| {
            let mut digits = line.chars()
                .filter(|c| c.is_ascii_digit());
            let first = digits.next().unwrap_or('0');
            let last = digits.last().unwrap_or(first);
            10u32 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap()
        })
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::solve;

    const TEST_DATA: &str = "\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    fn day1() {
        assert_eq!(
            solve(TEST_DATA).unwrap(),
            142,
        )
    }
}