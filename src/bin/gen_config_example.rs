use std::fs;
use waylyrics::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::write("config.toml", toml::to_string(&Config::default())?)?;
    Ok(())
}
