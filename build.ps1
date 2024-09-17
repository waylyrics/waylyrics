$Env:WAYLYRICS_THEME_PRESETS_DIR = "..\share\waylyrics\themes"
$Env:GETTEXT_DIR = "C:\gtk-build\gtk\x64\release"
cargo build --release --no-default-features -F tray-icon -F i18n -F import-lyric -F export-lyric
