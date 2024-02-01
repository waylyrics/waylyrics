
- [Install with package manager](#install-with-package-manager)
  - [Arch-based](#arch-based)
  - [NixOS](#nixos)
- [Prepare Dependencies](#prepare-dependencies)
  - [Debian-based](#debian-based)
  - [Arch-based](#arch-based-1)
  - [openSUSE/RHEL/Fedora..](#opensuserhelfedora)
- [Build](#build)
  - [With stable toolchain](#with-stable-toolchain)
  - [With nightly toolchain](#with-nightly-toolchain)
  - [Compiling Schema](#compiling-schema)
  - [Packging example](#packging-example)

Releases are avaliable in [Actions](https://github.com/waylyrics/waylyrics/actions/workflows/smoketest.yml)

# Install with package manager

## Arch-based

```bash
paru -S aur/waylyrics-git
```

[Waylyrics-git](https://github.com/archlinuxcn/repo/tree/master/archlinuxcn/waylyrics-git) is also avaliable in archlinuxcn.

## NixOS

check this [PR](https://github.com/NixOS/nixpkgs/pull/231984) for an outdated example

# Prepare Dependencies

## Debian-based

```bash
sudo apt-get install libssl-dev libgtk-4-dev libdbus-1-dev libmimalloc-dev
```

## Arch-based

```bash
paru -S gtk4 libxcb mimalloc
```

## openSUSE/RHEL/Fedora..

```
cargo libgraphene-devel gtk4-devel openssl-devel dbus-1-devel mimalloc-devel pango-devel
```

# Build

```bash
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
```

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

## Compiling Schema

You can install the schema locally:

```bash
mkdir -p ~/.local/share/glib-2.0/schemas
cp io.poly000.waylyrics.gschema.xml ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

## Packging example

An example packaging script:

```bash
sudo install -m644 io.poly000.waylyrics.gschema.xml /usr/share/glib-2.0/schemas/
sudo install -dm755 /usr/share/waylyrics/themes
sudo cp -r themes/* /usr/share/waylyrics/themes/
```