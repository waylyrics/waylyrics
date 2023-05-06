# Installation

For arch users, you could use an aur helper and run:

```bash
paru -S aur/waylyrics-git
```

[Waylyrics-git](https://github.com/archlinuxcn/repo/tree/master/archlinuxcn/waylyrics-git) is also avaliable in archlinuxcn.

## Build

First, you should have `gtk4(-dev)` installed. Meanwhile, `dbus` is required for MPRIS.

### With stable toolchain

* note: your rustc should be 1.52+

```bash
export RUSTC_BOOTSTRAP=1
cargo build --release --locked --target target
```

### With nightly toolchain

```bash
cargo +nightly build --release --locked --target target
```

Target binaries are placed in `target/release/`.
