use std::fs;
use waylyrics::config::Config;
use anyhow::Result;

fn main() -> Result<()> {
    fs::write("config.toml", toml::to_string(&Config::default())?)?;
    Ok(())
}
