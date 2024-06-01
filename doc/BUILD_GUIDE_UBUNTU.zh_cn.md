# Waylyrics Ubuntu用户编译指南 (2024.06.01)

## 目录
- [观前提示](#观前提示)
- [准备环境](#准备环境)
- [克隆源码](#克隆源码)
- [开始编译](#开始编译)
  - [使用 stable 工具链](#使用-stable-工具链)
  - [使用 nightly 工具链](#使用-nightly-工具链)
- [复制二进制包到可执行文件文件夹下](#复制二进制包到可执行文件文件夹下)
- [编译 schemas](#编译-schemas)
- [软件图标设置](#软件图标设置)
- [桌面图标设置](#桌面图标设置)
- [软件主题设置](#软件主题设置)
- [编译结束](#编译结束)
- [Waylyric 一些常见使用技巧](#waylyric-一些常见使用技巧)
  - [置顶歌词](#置顶歌词)

## 观前提示
这里所有的命令都是运行在普通用户下的，请理解每条命令的作用之后再运行。如果遇到权限问题，请适当使用 sudo 提权。

在编译开始之前，请保证你的网络通讯良好，否则很可能会在编译开始时卡住。

这篇文档默认全局安装 Waylyrics ，高级玩家可以将部分文件放入 ~/.local/share 中。

## 准备环境
```shell
sudo apt-get install git build-essential libssl-dev libgtk-4-dev libdbus-1-dev libmimalloc-dev gettext cargo
```
📣Tips：这里的 cargo 软件包会拉起 rustc 软件包，请注意这里的rustc 软件包版本需要 >= 1.73。高阶玩家可以自己搭建 rust 环境，不用从包管理器安装 cargo 。

## 克隆源码
我们需要在 Github 上克隆 Waylyrics 的源代码来进行编译。请保证网络连接通畅，非常情况请使用魔法🪄。
```shell
mkdir gittemp
cd gittemp
#这里 mkdir 命令创建了一个名为 gittemp 的文件夹，并且使用 cd 命令切换命令行工作目录，这样做的目的是为了克隆的项目不污染家目录。
git clone https://github.com/waylyrics/waylyrics.git
#使用 git 命令克隆（clone） Waylyrics 项目的源代码。
cd waylyrics
#使用 cd 命令切换命令行工作目录到项目文件夹。
export WAYLYRICS_THEME_PRESETS_DIR=/usr/share/waylyrics/themes
#设置 Waylyrics 主题目录的变量，后面会用到的
```
## 开始编译
编译的时候有 stable 标准工具链，或者 nightly 每日更新工具链可以选择（小白不推荐 nightly ）。

⚠️Waring：第三次提醒您，请确保您的网络通畅，前方下载工具链需要大量流量。

### 使用 stable 工具链
```shell
cargo build --release --locked --target-dir target
#使用默认设置开始编译，并且将编译后二进制包输出到 ./target/release/ 文件夹。
```

### 使用 nightly 工具链
```shell
cargo +nightly build --release --locked --target-dir target
#使用每日更新工具链开始编译，并且将编译后二进制包输出到 ./target/release/ 文件夹。
```

## 复制二进制包到可执行文件文件夹下
在 Linux 下， /bin 文件夹下放着很多可执行文件，比如 apt 、 grep 等命令，我们把 waylyrics 的二进制文件复制到 /bin 目录下，这样我们就可以直接使用 waylyrics 命令启动 Waylyrics 程序。
```shell
sudo cp ./target/release/waylyrics /bin
#复制 waylyrics 二进制文件到 /bin 目录下
sudo chmod 755 /bin/waylyrics
#设置 /bin/waylyrics 的权限为755
```

## 编译 schemas
Waylyric 需要通过 GSettings API（GNOME桌面环境设置存储和检索系统的调用接口，因为 Waylyric 是 GTK 软件，所以需要调用此接口） 来允许应用程序读取和写入用户设置，让我们来开始编译 schemas 吧！
```shell
cp ./metainfo/io.github.waylyrics.Waylyrics.gschema.xml /usr/share/glib-2.0/schemas
#复制 schemas 文件到 /usr/share/glib-2.0/schemas/ 目录下
glib-compile-schemas /usr/share/glib-2.0/schemas/
#编译 schemas 文件
```

## 多语言设置
📣Tips：此项为可选项，并不会影响软件正常使用
```bash
cd locales
for po in $(find . -type f -name '*.po')
do
    mkdir -p ~/.local/share/locale/${po#/*}
    msgfmt -o ~/.local/share/locale/${po%.po}.mo ${po}
done
```

## 软件图标设置
📣Tips：此项很重要，会影响软件的正常使用体验

我们要给软件设置一个好看的 Logo ！当然是我们可爱的 Waylyrics 小Fu狸！让我们把可爱的小Fu狸带到系统中吧！
```shell
cp -r ./res/icons/* /usr/share/icons/
#将 ./res/icons/ 下所有的文件复制到系统指定的图标文件夹
```

## 桌面图标设置
📣Tips：此项很重要，会影响软件的正常使用体验

我们需要在桌面有一个启动图标来操作 Waylyrics ，正好源码里面给我们提供了现成可用的 desktop 文件来使用，我们只需要将此文件复制到存储桌面图标的文件夹中就可以了。
```shell
#请确认命令行的工作目录在项目目录下，而不是在子目录下，如 locales , metainfo 等文件夹
cp ./metainfo/io.github.waylyrics.Waylyrics.desktop /usr/share/applications/
#复制 Waylyrics 的桌面文件到 /usr/share/applications/
sudo chmod 644 /usr/share/applications/io.github.waylyrics.Waylyrics.desktop
#设置桌面图标的权限为 644
```
## 软件主题设置
📣Tips：此项为可选项，并不会影响软件正常使用

安装完软件后还需要漂亮的主题，正好 Waylyric 支持主题配置，我们需要把默认主题复制到标准目录下。
```shell
mkdir -p /usr/share/waylyrics/themes/
#创建 /usr/share/waylyrics/themes/ 这一串文件夹
cp -r ./themes/* /usr/share/waylyrics/themes/
#复制 ./themes/ 下所有文件到 /usr/share/waylyrics/themes/
sudo chmod 755 -R /usr/share/waylyrics/themes/
#设置文件夹权限为755
```

## 编译结束
至此，你也应该完成了全部的安装过程了，是不是感觉很有趣呢？但是一切已经过去，软件的编译过程已经完成！

## Waylyric 一些常见使用技巧
### 置顶歌词
保持 Waylyrics 窗口状态，右键其标题栏，选择更多，选中置顶，然后关闭其窗口状态就可以实现歌词置顶。