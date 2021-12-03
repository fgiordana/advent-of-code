use anyhow::Result;
use std::fs;


pub const FILEPATH: &str = "data/day_03/input.txt";
const TREE: char = '#';
const SLOPES: &[(usize, usize)] = &[
	(1, 1),
	(3, 1),
	(5, 1),
	(7, 1),
	(1, 2)
];


pub fn part1() -> Result<usize> {
	Ok(fs::read_to_string(FILEPATH)?
		.split('\n')
		.fold((0usize, 0usize), |(n, pos), s| {
			(n + (s.chars().nth(pos % s.len()).unwrap() == TREE) as usize, pos + 3)
		}).0)
}

pub fn part2() -> Result<usize> {
	let content = fs::read_to_string(FILEPATH)?;
	let terrain = content.split('\n')
		.collect::<Vec<&str>>();
	Ok(SLOPES.iter()
		.map(|&(r, d)| {
			terrain.iter()
				.step_by(d)
				.fold((0usize, 0usize), |(n, pos), s| {
					(n + (s.chars().nth(pos % s.len()).unwrap() == TREE) as usize, pos + r)	
				}).0
		})
		.product())
}


#[cfg(test)]
mod test {

	use super::*;

	const PART1_RESULT: usize = 162;
	const PART2_RESULT: usize = 3064612320;

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