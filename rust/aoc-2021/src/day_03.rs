use anyhow::Result;
use std::fs;

const FILEPATH: &str = "data/day_03/input.txt";

fn bits_to_decimal(bits: &[bool]) -> u64 {
    bits.iter().rev().enumerate().fold(0u64, |sum, (i, &bit)| {
        sum + (bit as u64) * 2u64.pow(i as u32)
    })
}

fn negate_bits(bits: &[bool]) -> Vec<bool> {
    bits.iter().map(|&x| !x).collect()
}

pub fn part1() -> Result<u64> {
    let content = fs::read_to_string(FILEPATH)?;
    let lines = content.split('\n').collect::<Vec<&str>>();
    let size = lines[0].len();
    let length = lines.len();
    let gamma_bits: Vec<bool> = lines
        .iter()
        .map(|&s| {
            let mut bits = vec![false; size];
            s.chars().enumerate().for_each(|(i, c)| bits[i] = c == '1');
            bits
        })
        .fold(vec![0usize; size], |ones, bits| {
            ones.iter()
                .zip(bits.iter())
                .map(|(&o, &b)| o + b as usize)
                .collect()
        })
        .iter()
        .map(|&x| x > length / 2)
        .collect();

    let gamma = bits_to_decimal(&gamma_bits);
    let epsilon_bits = negate_bits(&gamma_bits);
    let epsilon = bits_to_decimal(&epsilon_bits);

    Ok(gamma * epsilon)
}

fn gamma_bit(numbers: &[&Vec<bool>], pos: usize) -> bool {
    numbers
        .iter()
        .map(|&number| number[pos] as usize)
        .sum::<usize>()
        >= (numbers.len() as f32 / 2.0f32).ceil() as usize
}

fn filter_numbers<'a>(numbers: &[&'a Vec<bool>], pos: usize, flip: bool) -> Vec<&'a Vec<bool>> {
    let mut bit = gamma_bit(numbers, pos);
    if flip {
        bit = !bit;
    }
    let filtered = numbers
        .iter()
        .copied()
        .filter(|n| n[pos] != bit)
        .collect::<Vec<&Vec<bool>>>();
    if filtered.len() > 1 {
        filter_numbers(&filtered, pos + 1, flip)
    } else {
        filtered
    }
}

pub fn part2() -> Result<u64> {
    let content = fs::read_to_string(FILEPATH)?;
    let lines = content.split('\n').collect::<Vec<&str>>();
    let size = lines[0].len();

    let bit_numbers: Vec<Vec<bool>> = lines
        .iter()
        .map(|&s| {
            let mut bits = vec![false; size];
            s.chars().enumerate().for_each(|(i, c)| bits[i] = c == '1');
            bits
        })
        .collect();

    let ogr = bits_to_decimal(
        filter_numbers(&bit_numbers.iter().collect::<Vec<&Vec<bool>>>(), 0, false)[0],
    );
    let csr = bits_to_decimal(
        filter_numbers(&bit_numbers.iter().collect::<Vec<&Vec<bool>>>(), 0, true)[0],
    );
    Ok(ogr * csr)
}

#[cfg(test)]
mod test {

    use super::*;

    const PART1_RESULT: u64 = 3687446;
    const PART2_RESULT: u64 = 4406844;

    #[test]
    fn test_bits_to_decimal() {
        let bits = [true, true, false, false, true];
        assert_eq!(bits_to_decimal(&bits), 25);
    }

    #[test]
    fn test_negate_bits() {
        let bits = [true, true, false, false, true];
        assert_eq!(negate_bits(&bits), [false, false, true, true, false]);
    }

    #[test]
    fn check_part1() {
        let result = part1().unwrap();
        assert_eq!(result, PART1_RESULT);
    }

    #[test]
    fn test_gamma_bit() {
        let numbers = [
            &vec![true, false, true],
            &vec![true, true, false],
            &vec![false, false, false],
        ];
        assert!(gamma_bit(&numbers, 0));
        assert!(!gamma_bit(&numbers, 1));
        assert!(!gamma_bit(&numbers, 2));

        let numbers = [&vec![true, false, false], &vec![true, true, false]];
        assert!(gamma_bit(&numbers, 0));
        assert!(gamma_bit(&numbers, 1));
        assert!(!gamma_bit(&numbers, 2));
    }

    #[test]
    fn check_part2() {
        let result = part2().unwrap();
        assert_eq!(result, PART2_RESULT);
    }
}
