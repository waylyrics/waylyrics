use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("CARGO_CFG_TARGET_OS")?.as_str() == "windows" {
        #[cfg(windows)]
        {
            use std::path::Path;
            windows_exe_info::icon::icon_ico(Path::new("res/win/icon.ico"));
            windows_exe_info::versioninfo::VersionInfo::from_cargo_env_ex(
                Some("Desktop lyric for SMTC"),
                Some("Waylyrics"),
                None,
                None,
            )
            .link()?;
        }
    }

    Ok(())
}
