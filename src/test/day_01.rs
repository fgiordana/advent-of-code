use std::path::Path;

use crate::day_01::{day_01, FILEPATH, TARGET_SUM};

#[test]
fn input_file_exists() {
	assert!(Path::new(FILEPATH).exists());
}

#[test]
fn sum_equals_target() {
	let (x, y) = day_01().unwrap();
	assert_eq!(x + y, TARGET_SUM);
}
