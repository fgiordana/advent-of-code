use std::path::Path;

use crate::day_01::{part1, part2, FILEPATH};

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
