use std::fs;
use anyhow::Result;
use phf::phf_map;


const FILEPATH: &str = "data/day_01/input.txt";

const DIGIT_MAP: phf::Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9'
};

pub fn run() -> Result<()> {
   let input = fs::read_to_string(FILEPATH)?;
   println!("Part1: {}", part1(&input)?);
   println!("Part2: {}", part2(&input)?);

   Ok(())
}

pub fn part1(input: &str) -> Result<u32> {

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

pub fn part2(input: &str) -> Result<u32> {

    let result = input.split('\n')
        .map(to_digits)
        .map(|digits| {
            let first = digits.first().unwrap_or(&0u32);
            let last = digits.last().unwrap_or(first);
            10u32 * first + last
        })
        .sum::<u32>();

    Ok(result)
}

fn to_digits(input: &str) -> Vec<u32> {
    let mut digits = vec![];
    for (pos, c) in input.char_indices() {
        for key in DIGIT_MAP.keys() {
            if input[pos..].starts_with(key) {
                digits.push(*DIGIT_MAP.get(key).unwrap());
                break;
            }
        }
        if c.is_ascii_digit() {
            digits.push(c)
        }
    }
    digits.iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}



#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA_PART1: &str = "\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(TEST_DATA_PART1).unwrap(),
            142,
        )
    }

    const TEST_DATA_PART2: &str = 
        "two1nine\n\
         eightwothree\n\
         abcone2threexyz\n\
         xtwone3four\n\
         4nineeightseven2\n\
         zoneight234\n\
         7pqrstsixteen";
    
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(TEST_DATA_PART2).unwrap(),
            281,
        )
    }

    #[test]
    fn test_to_digits() {
        const INPUT: &str = "zoneight234";
        assert_eq!(
            to_digits(INPUT),
            [1, 8, 2, 3, 4]
        );
    }
}