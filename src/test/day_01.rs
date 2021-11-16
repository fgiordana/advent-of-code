use std::path::Path;

use crate::day_01::{day_01, FILEPATH};

const EXPECTED_RESULT: i64 = 55776;

#[test]
fn input_file_exists() {
	assert!(Path::new(FILEPATH).exists());
}

#[test]
fn check_result() {
	let result = day_01().unwrap();
	assert_eq!(result, EXPECTED_RESULT);
}
