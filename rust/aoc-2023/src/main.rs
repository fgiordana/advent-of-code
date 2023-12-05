use anyhow::Result;

mod day_01;

fn main() -> Result<()> {
    println!("Day 1");
    let result = day_01::run()?;
    println!("Result: {result}");

    Ok(())
}
