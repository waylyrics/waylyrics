$Env:WAYLYRICS_THEME_PRESETS_DIR = "..\share\waylyrics\themes"
# to build i18n, you may have to switch to msys environment
# I tried to not use `--force-local` for tar but gettext-sys
# failed to build with MSVC, and I don't know how to write a
# pkg-config file in gtk-build\gtk\x64\release\lib\pkgconfig
cargo build --release --no-default-features -F tray-icon
