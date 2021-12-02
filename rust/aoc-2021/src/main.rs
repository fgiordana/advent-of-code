use anyhow::Result;
use flexi_logger::Logger;
use log::{error, info};

mod day_01;

fn main() -> Result<()> {
	Logger::try_with_str("info")?.start()?;

    info!("Advent of Code 2021!");

	info!("DAY 01");
	info!("Part 1");
	match day_01::part1() {
		Ok(result) => info!("RESULT: {}", result),
		Err(e) => error!("Error: {}", e)
	}
	info!("Part 2");
	match day_01::part2() {
		Ok(result) => info!("RESULT: {}", result),
		Err(e) => error!("Error: {}", e)
	}
	info!("");

	Ok(())
}
