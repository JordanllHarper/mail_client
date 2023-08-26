pub mod api;
pub mod models;
use color_eyre::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    color_eyre::install()?;

    Ok(())
}
