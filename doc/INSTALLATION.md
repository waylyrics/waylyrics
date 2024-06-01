
- [下载预编译二进制](#下载预编译二进制)
- [通过包管理器安装](#通过包管理器安装)
- [安装构建依赖](#安装构建依赖)
  - [Debian-based](#debian-based)
  - [Arch-based](#arch-based)
  - [其他RPM系发行版：](#其他rpm系发行版)
  - [Windows](#windows)
- [编译](#编译)
  - [使用 stable 工具链](#使用-stable-工具链)
  - [使用 nightly 工具链](#使用-nightly-工具链)
  - [本地安装](#本地安装)
    - [编译Schema](#编译schema)
    - [本地化文件](#本地化文件)
    - [Desktop 文件](#desktop-文件)
  - [打包](#打包)

可以在 [Actions](https://github.com/waylyrics/waylyrics/actions/workflows/smoketest.yml) 下载发布

# 下载预编译二进制

[builds]: https://github.com/waylyrics/waylyrics/actions/workflows/test.yml

我们在 [github action][builds] 提供下载。

这些构建将 `WAYLYRICS_THEME_PRESETS_DIR` 设置为 `/usr/share/waylyrics/themes`，

你可以把主题放在 `${XDG_DATA_HOME}/_themes/`，waylyrics 会先尝试从这里加载。

# 通过包管理器安装

[![Packaging status](https://repology.org/badge/vertical-allrepos/waylyrics.svg)](https://repology.org/project/waylyrics/versions)

## Fedora Copr

Fedora 用户可以使用 [yohane-shiro/waylyrics](https://copr.fedorainfracloud.org/coprs/yohane-shiro/waylyrics)

## archlinuxcn

Arch Linux 用户可以使用 [archlinuxcn](https://github.com/archlinuxcn/repo) 源安装

## Flatpak

<a href='https://flathub.org/apps/io.github.waylyrics.Waylyrics'>
    <img width='240' alt='Download on Flathub' src='https://flathub.org/api/badge?locale=zh-Hans'/>
</a>

## Spark Store (Ubuntu 22.04 LTS)

<a href='https://www.spark-app.store/'>
    <img width='120' alt='去星火商店下载' src='https://gitee.com/spark-store-project/spark-store/raw/dev/src/assets/tags/community.png'/>
</a>

Ubuntu 22.04 用户可以去星火商店安装，其他版本没有测试。
```shell
sudo aptss install waylyrics
```

# 安装构建依赖

## Debian-based

```bash
sudo apt-get install libssl-dev libgtk-4-dev libdbus-1-dev libmimalloc-dev gettext cargo
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

## Windows

请查阅 [gtk book](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_windows.html#install-gtk-4) 安装 gtk4

如果要使用 MSVC 请启用 `--no-default-features` ，gettext-rs 不支持 Windows MSVC 编译

对于 `opencc` ，则需要你复制他们的预构建发布至 `%systemdrive%\gtk-build\gtk\x64\release` 。

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

生成的二进制会被放在 `./target/release/`

## 本地安装

### 编译Schema

```bash
install -Dm644 metainfo/io.github.waylyrics.Waylyrics.gschema.xml -t ~/.local/share/glib-2.0/schemas/
glib-compile-schemas ~/.local/share/glib-2.0/schemas/
```

### 本地化文件

```bash
cd locales
for po in $(find . -type f -name '*.po')
do
    mkdir -p ~/.local/share/locale/${po#/*}
    msgfmt -o ~/.local/share/locale/${po%.po}.mo ${po}
done
```

### Desktop 文件

```bash
install -Dm644 metainfo/io.github.waylyrics.Waylyrics.desktop -t ~/.local/share/applications
```

## 打包

打包脚本样例：

```bash
install -Dm644 ./metainfo/io.github.waylyrics.Waylyrics.gschema.xml -t /usr/share/glib-2.0/schemas/
install -Dm644 ./metainfo/"io.github.waylyrics.Waylyrics.desktop" -t /usr/share/applications/
install -dm755 /usr/share/waylyrics/themes
cp -r ./themes/* /usr/share/waylyrics/themes/
cp -r ./res/icons/hicolor /usr/share/icons/

cd locales
for po in $(find . -type f -name '*.po')
do
    mkdir -p /usr/share/locale/${po#/*}
    msgfmt -o /usr/share/locale/${po%.po}.mo ${po}
done
```
