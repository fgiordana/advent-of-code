use anyhow::Result;
use flexi_logger::Logger;
use log::{error, info};

mod day_01;
mod test;

use day_01::day_01;

fn main() -> Result<()> {
    Logger::try_with_str("info")?.start()?;

    info!("Advent of Code 2020!");

    info!("DAY 01");
    match day_01() {
        Ok(result) => info!("RESULT: {}", result),
        Err(e) => error!("ERROR: {}", e)
    }

    Ok(())
}
