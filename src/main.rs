use anyhow::Result;
use flexi_logger::Logger;
use log::{error, info};

mod day_01;
mod day_02;


fn main() -> Result<()> {
    Logger::try_with_str("info")?.start()?;

    info!("Advent of Code 2020!");

    info!("DAY 01");
    info!("Part 1");
    match day_01::part1() {
        Ok(result) => info!("RESULT: {}", result),
        Err(e) => error!("ERROR: {}", e),
    }
    info!("Part 2");
    match day_01::part2() {
        Ok(result) => info!("RESULT: {}", result),
        Err(e) => error!("ERROR: {}", e),
    }
	info!("");

	info!("DAY 02");
	info!("Part 1");
	match day_02::part1() {
		Ok(result) => info!("RESULT: {}", result),
		Err(e) => error!("ERROR: {}", e),
	}
	info!("");
	
	Ok(())
}
