use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS")?.as_str() == "windows" {
        windows_exe_info::icon::icon_ico("res/win/icon.ico");
        windows_exe_info::versioninfo::VersionInfo::from_cargo_env_ex(
            Some("Desktop lyric for SMTC"),
            Some("Waylyrics"),
            None,
            None,
        )
        .link()?;
    }

    Ok(())
}
