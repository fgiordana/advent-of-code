use anyhow::Result;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

fn main() -> Result<()> {
    println!("Day 01");
    day_01::run()?;
    println!("Day 02");
    day_02::run()?;
    println!("Day 03");
    day_03::run()?;
    println!("Day 04");
    day_04::run()?;

    Ok(())
}
