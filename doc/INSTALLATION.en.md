
- [Download pre-built executable](#download-pre-built-executable)
- [Install with package manager](#install-with-package-manager)
- [Prepare Dependencies](#prepare-dependencies)
  - [Debian-based](#debian-based)
  - [Arch-based](#arch-based)
  - [Other RPM-based disturbution](#other-rpm-based-disturbution)
  - [Windows](#windows)
- [Build](#build)
  - [With stable toolchain](#with-stable-toolchain)
  - [With nightly toolchain](#with-nightly-toolchain)
  - [Local install](#local-install)
    - [Compiling Schema](#compiling-schema)
    - [Desktop File](#desktop-file)
  - [Packging example](#packging-example)

Releases are avaliable in [Actions](https://github.com/waylyrics/waylyrics/actions/workflows/smoketest.yml)

# Download pre-built executable

[builds]: https://github.com/waylyrics/waylyrics/actions/workflows/test.yml

We provide builds in [github action][builds].

Note that these build sets `WAYLYRICS_THEME_PRESETS_DIR` as `/usr/share/waylyrics/themes`,

You can also place themes to `${XDG_DATA_HOME}/_themes/`, waylyrics will try this first.

# Install with package manager

[![Packaging status](https://repology.org/badge/vertical-allrepos/waylyrics.svg)](https://repology.org/project/waylyrics/versions)

Fedora users may use [yohane-shiro/waylyrics](https://copr.fedorainfracloud.org/coprs/yohane-shiro/waylyrics/) in Fedora Copr.

# Prepare Dependencies

## Debian-based

```bash
sudo apt-get install libssl-dev libgtk-4-dev libdbus-1-dev libmimalloc-dev gettext
```

## Arch-based

```bash
paru -S gtk4 libxcb mimalloc
```

## Other RPM-based disturbution

Please install dependencies in below:

```
cargo libgraphene-devel gtk4-devel openssl-devel dbus-1-devel mimalloc-devel pango-devel gettext
```

## Windows

To setup gtk4, please check [gtk book](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html#install-gtk-4).

Also you need to pass `--no-default-features` if not using a `-gnu` target because gettext-rs doesn't support Windows MSVC for now.

For `opencc`, you should copy their precompiled release to `%systemdrive%\gtk-build\gtk\x64\release`

# Build

```bash
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
```

waylyrics will load themes from `PRESET_DIR`, unless they were overridden by `${XDG_DATA_HOME}/_themes/<name>.css` 

If `WAYLYRICS_THEME_PRESETS_DIR` is not set, waylyrics will only load themes from user data home.

## With stable toolchain

* note: your rustc should be 1.73+

```bash
cargo build --release --locked --target-dir target
```

## With nightly toolchain

```bash
cargo +nightly build --release --locked --target-dir target
```

Target binaries are placed in `target/release/`.

## Local install

### Compiling Schema

```bash
install -Dm644 metainfo/io.github.waylyrics.Waylyrics.gschema.xml -t ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

### Desktop File

```bash
install -Dm644 metainfo/io.github.waylyrics.Waylyrics.desktop -t ~/.local/share/applications
```

## Packging example

An example packaging script:

```bash
install -Dm644 metainfo/io.github.waylyrics.Waylyrics.gschema.xml -t /usr/share/glib-2.0/schemas/
install -Dm644 metainfo/io.github.waylyrics.Waylyrics.desktop -t /usr/share/applications
install -dm755 /usr/share/waylyrics/themes
cp -r themes/* /usr/share/waylyrics/themes/
cp -r res/icons /usr/share/icons

cd locales
for po in $(find . -type f -name '*.po')
do
    mkdir -p /usr/share/local/share/locale/${po#/*}
    msgfmt -o /usr/share/local/share/locale/${po%.po}.mo ${po}
done
```