use anyhow::Result;

mod day_01;
mod day_02;

fn main() -> Result<()> {
    day_01::run()?;
    day_02::run()?;

    Ok(())
}
