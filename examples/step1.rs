use anyhow::{Context, Result};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config_file = args.get(1).context("Config file must be provided")?;
    let config_string = std::fs::read_to_string(config_file)?;
    println!("{config_string}");
    Ok(())
}
