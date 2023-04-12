use waylyrics::config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::write("config.toml.example", toml::to_string(&Config::default())?)?;
    Ok(())
}
