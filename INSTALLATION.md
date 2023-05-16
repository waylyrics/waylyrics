
- [Install with package manager](#install-with-package-manager)
  - [Arch-based](#arch-based)
  - [NixOS](#nixos)
- [Prepare Dependencies](#prepare-dependencies)
  - [Debian-based](#debian-based)
  - [Arch-based](#arch-based-1)
- [Build](#build)
  - [With stable toolchain](#with-stable-toolchain)
  - [With nightly toolchain](#with-nightly-toolchain)
  - [Packging example](#packging-example)


# Install with package manager

## Arch-based

```bash
paru -S aur/waylyrics-git
```

[Waylyrics-git](https://github.com/archlinuxcn/repo/tree/master/archlinuxcn/waylyrics-git) is also avaliable in archlinuxcn.

## NixOS

After the [PR](https://github.com/NixOS/nixpkgs/pull/231984) merged:

```
nix-env -iA 
```

# Prepare Dependencies

## Debian-based

```bash
sudo apt-get install libssl-dev libgtk-4-dev libdbus-1-dev
```

## Arch-based

```bash
paru -S gtk4 libxcb
```

# Build

```bash
export WAYLYRICS_DEFAULT_CONFIG=/usr/share/waylyrics/config.toml
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
```

## With stable toolchain

* note: your rustc should be 1.52+

```bash
export RUSTC_BOOTSTRAP=1
cargo build --release --locked --target target
```

## With nightly toolchain

```bash
cargo +nightly build --release --locked --target target
```

Target binaries are placed in `target/release/`.

## Packging example

An example packaging script:

```bash
cargo run --bin gen_config_example
sudo install -m644 config.toml /usr/share/waylyrics/config.toml
sudo install -Dm755 /usr/share/waylyrics/themes
sudo cp -r themes/* /usr/share/waylyrics/themes/
```