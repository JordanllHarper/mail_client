pub mod api;
use color_eyre::Result;
extern crate google_signin;

fn main() -> Result<()> {
    println!("Hello, world!");

    color_eyre::install()?;

    Ok(())
}
