
- [下载预编译二进制](#下载预编译二进制)
- [通过包管理器安装](#通过包管理器安装)
- [安装构建依赖](#安装构建依赖)
  - [Debian-based](#debian-based)
  - [Arch-based](#arch-based)
  - [其他RPM系发行版：](#其他rpm系发行版)
- [编译](#编译)
  - [使用 stable 工具链](#使用-stable-工具链)
  - [使用 nightly 工具链](#使用-nightly-工具链)
  - [编译Schema](#编译schema)
  - [打包](#打包)

可以在 [Actions](https://github.com/waylyrics/waylyrics/actions/workflows/smoketest.yml) 下载发布

# 下载预编译二进制

[builds]: https://github.com/waylyrics/waylyrics/actions/workflows/test.yml

我们在 [github action][builds] 提供下载。

这些构建将 `WAYLYRICS_THEME_PRESETS_DIR` 设置为 `/usr/share/waylyrics/themes`，

你可以把主题放在 `${XDG_DATA_HOME}/_themes/`，waylyrics 会先尝试从这里加载。

# 通过包管理器安装

[![Packaging status](https://repology.org/badge/vertical-allrepos/waylyrics.svg)](https://repology.org/project/waylyrics/versions)

# 安装构建依赖

## Debian-based

```bash
sudo apt-get install libssl-dev libgtk-4-dev libdbus-1-dev libmimalloc-dev gettext
```

## Arch-based

```bash
paru -S gtk4 libxcb mimalloc
```

## 其他RPM系发行版：

请安装如下依赖：

```
cargo libgraphene-devel gtk4-devel openssl-devel dbus-1-devel mimalloc-devel pango-devel gettext
```

# 编译

```bash
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
```

waylyrics 会从该位置加载主题，除非被 `${XDG_DATA_HOME}/_themes/<name>.css` 覆盖

如果编译时没有设置这个环境变量，waylyrics将只能加载用户主题。

## 使用 stable 工具链

* note: 你的 rustc 需要在 1.73+

```bash
cargo build --release --locked --target-dir target
```

## 使用 nightly 工具链

```bash
cargo +nightly build --release --locked --target-dir target
```

生成的二进制会被放在 `target/release/`

## 编译Schema

本地安装schema:

```bash
mkdir -p ~/.local/share/glib-2.0/schemas
cp io.poly000.waylyrics.gschema.xml ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

## 打包

打包脚本样例：

```bash
sudo install -m644 io.poly000.waylyrics.gschema.xml /usr/share/glib-2.0/schemas/
sudo install -dm755 /usr/share/waylyrics/themes
sudo cp -r themes/* /usr/share/waylyrics/themes/
```