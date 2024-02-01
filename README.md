# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/+FWgnE0GRDYZhNjc1)
[![matrix-group](https://img.shields.io/matrix/waylyrics_x:catgirl.cloud.svg?server_fqdn=matrix.catgirl.cloud)](https://matrix.to/#/#waylyrics_x:catgirl.cloud)

桌面歌词，基于GTK4，made with ❤

![](https://github.com/waylyrics/waylyrics/assets/34085039/dd7d9236-b2ae-47da-b4a3-e19a7d10e31b)

- [Waylyrics](#waylyrics)
  - [构建/安装](#构建安装)
  - [用法](#用法)
  - [依赖](#依赖)
    - [推荐的播放器](#推荐的播放器)
      - [在线](#在线)
        - [GTK+](#gtk)
        - [Qt](#qt)
        - [Electron](#electron)
      - [本地](#本地)
        - [GTK+](#gtk-1)
        - [原生](#原生)
        - [Qt](#qt-1)
    - [无法使用的播放器](#无法使用的播放器)
  - [目录](#目录)
  - [替代品](#替代品)
    - [Linux](#linux)
    - [Windows](#windows)
  - [Credit](#credit)
  - [License](#license)

## 构建/安装

详阅 [INSTALLATION.md](doc/INSTALLATION.md)

## 用法

https://github.com/waylyrics/waylyrics/blob/d2132b42b135e0de09640de6a5a0b4797871c67e/src/config.rs#L67-L71

SIGUSR1: 断开当前播放器

SIGUSR2: 开关GTK CSD

## 依赖

- 播放器需至少在MPRIS提供 Position，Metadata(有title)，PlaybackStatus
- 需要你的wm允许你设置窗口置顶

### 推荐的播放器

#### 在线

> Qcm, Feeluown-{netease, qqmusic}, ElectronNCM, YesPlayMusic(OSD)，可以直接拿歌曲id

##### GTK+

- [netease-cloud-music-gtk](https://github.com/gmg137/netease-cloud-music-gtk)

##### Qt

- [FeelUOwn](https://github.com/feeluown/FeelUOwn/), 3.9.12+
- [Qcm](https://github.com/hypengw/Qcm)
- [Telegram](https://t.me/Music163Bot)

##### Electron

- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [lx-music-desktop](https://github.com/lyswhut/lx-music-desktop)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)
- [youtube-music](https://github.com/th-ch/youtube-music), 歌多，有mv

#### 本地

##### GTK+

- [amberol](https://gitlab.gnome.org/World/amberol)

##### 原生

- mpv + [mpv-mpris](https://github.com/hoyon/mpv-mpris)

##### Qt

- [VLC](https://www.videolan.org)

### 无法使用的播放器

[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[flutter-netease-music]: https://github.com/boyan01/flutter-netease-music
[youtube-music]: https://github.com/th-ch/youtube-music


| Player                  | OSD | issue              |
| ----------------------- | --- | ------------------ |
| Firefox                 | X   | 不提供播放位置调用 |
| qqmusic                 | O   | 位置一直为0        |
| [flutter-netease-music] | X   | 完全不支持mpris    |

## 目录

一般情况会创建的目录（可能被用户XDG设置影响）

```
~/.cache/waylyrics/XX/...
~/.config/waylyrics/...
# waylyrics 会首先尝试在这里加载主题，找不到的话就从全局模板目录找
~/.local/share/waylyrics/_themes/...
```

## 替代品

[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics
[lx-music-desktop]: https://github.com/lyswhut/lx-music-desktop
[Sunamu]: https://github.com/NyaomiDEV/Sunamu
[lyricsSeeker]: https://github.com/BruceZhang1993/LyricsSeeker
[caraoke-plasmoid]: https://github.com/Copay/caraoke-plasmoid
[desktop-lyric]: https://github.com/tuberry/desktop-lyric
[可道歌词]: https://www.autolyric.com/
[Lyricify]: https://github.com/WXRIW/Lyricify-App

### Linux

Sway用户可以看看 [waybar-netease-music-lyrics].

[lx-music-desktop] 多来源且带有桌面歌词支持

[Sunamu] 可以启动服务器或电子app，它的歌词显示效果更好，并且比waylyrics的功能更多

如果你是 GNOME 用户，你可以看看 [desktop-lyric]

BruceZhang1993's [lyricsSeeker] is still WIP, but it may have better-looking and better integration with KDE.

Copay's [caraoke-plasmoid] is currently Plasma-only, though it is easy to remove plasmoid components

### Windows

[可道歌词] 只支持一些本地播放器

[Lyricify] 支持一些在线播放器，4版只支持 Spotify

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
