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

另见 [desktop文件](io.poly000.waylyrics.desktop)

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
[Sunamu]: https://github.com/NyaomiDEV/Sunamu
[lyricsSeeker]: https://github.com/BruceZhang1993/LyricsSeeker
[caraoke-plasmoid]: https://github.com/Copay/caraoke-plasmoid
[desktop-lyric]: https://github.com/tuberry/desktop-lyric
[可道歌词]: https://www.autolyric.com/
[Lyricify]: https://github.com/WXRIW/Lyricify-App

### Linux

| 名称                          | 技术栈      | 桌面支持   | 播放器支持 | 备注 |
| ----------------------------- | ----------- | ---------- | ---------- | ---- |
| [waybar-netease-music-lyrics] | bash        | Sway       | NCM[^0]    |      |
| [Sunamu]                      | Electron/TS | X/Xwayland | MPRIS      |      |
| [desktop-lyric]               | JavaScript  | GNOME      | MPRIS      |      |
| [lyricsSeeker]                | Python+QML  | Plasma     |            | WIP  |
| [caraoke-plasmoid]            | QML         | Plasma     | MPRIS      |      |

[^0]: https://github.com/kangxiaoju/waybar-netease-music-lyrics/blob/f84810fe1628ca38fa36d88506152d88cf0e77e4/song.sh#L41-L59

### Windows

| 名称       | 技术栈               | 播放器支持                                                                                          | 备注 |
| ---------- | -------------------- | --------------------------------------------------------------------------------------------------- | ---- |
| [可道歌词] | Unknown              | Windows Media Player<br>Foobar2000<br>AIMP<br>Winamp                                                |      |
| [Lyricify] | UWP/C#(4+)<br>C#(3-) | Spotify<br>iTunes<br>Apple Music<br>Music Center<br>QQ Music<br>Netease Cloud Music<br>YesPlayMusic |      |

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
