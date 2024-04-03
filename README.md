# Waylyrics

<p align="center">
  <img src="img/waylyrics.png" style="height: 30vw"></img>
  <br />
  <a href="https://t.me/+FWgnE0GRDYZhNjc1">Telegram</a>&nbsp;|&nbsp;
  <a href="https://matrix.to/#/#waylyrics_x:catgirl.cloud">Matrix</a>
  <br />
  <a href="https://www.bilibili.com/video/BV1ap421R7nD" target="blank">功能预览</a>
</p>

- [Waylyrics](#waylyrics)
  - [构建/安装](#构建安装)
  - [用法](#用法)
  - [依赖](#依赖)
  - [推荐的播放器](#推荐的播放器)
  - [无法使用的播放器](#无法使用的播放器)
  - [目录](#目录)
    - [Linux/Unix](#linuxunix)
    - [Windows](#windows)
  - [替代品](#替代品)
    - [Linux](#linux)
    - [Windows](#windows-1)
  - [Credit](#credit)
  - [License](#license)

## 构建/安装

详阅 [INSTALLATION.md](doc/INSTALLATION.md)

## 用法

另见 [desktop文件](io.poly000.waylyrics.desktop)

https://github.com/waylyrics/waylyrics/blob/d2132b42b135e0de09640de6a5a0b4797871c67e/src/config.rs#L67-L71

## 依赖

- 播放器需至少在MPRIS提供 Position，Metadata(有title)，PlaybackStatus
- 需要你的wm允许你设置窗口置顶

## 推荐的播放器

[插件]: https://addons.mozilla.org/en-US/firefox/addon/plasma-integration/
[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[amberol]: https://gitlab.gnome.org/World/amberol
[lollypop]: https://github.com/hamonikr/lollypop

[FeelUOwn]: https://github.com/feeluown/FeelUOwn/
[Qcm]: https://github.com/hypengw/Qcm
[Telegram]: https://t.me/Music163Bot
[VLC]: https://www.videolan.org

[Electron-NCM]: https://github.com/Rocket1184/electron-netease-cloud-music
[YesPlayMusic]: https://github.com/qier222/YesPlayMusic
[youtube-music]: https://github.com/th-ch/youtube-music
[go-musicfox]: https://github.com/go-musicfox/go-musicfox
[fb7e486]: https://github.com/go-musicfox/go-musicfox/commit/fb7e4865a39c9537f3868d62dae7c8690a9ca8c4

[mpv-mpris]: https://github.com/hoyon/mpv-mpris

| 名称                      | 在线 | 框架      | 备注                       |
| ------------------------- | ---- | --------- | -------------------------- |
| Firefox                   | O    | 浏览器    | 需要[插件]                 |
| Chrome                    | O    | 浏览器    |                            |
| [netease-cloud-music-gtk] | O    | GTK4      | 2.3.0+                     |
| [amberol]                 | X    | GTK4      |                            |
| [lollypop]                | X    | GTK4      |                            |
| [FeelUOwn]                | O    | Qt5       | 3.9.12+                    |
| [VLC]                     | X    | Qt5       |                            |
| [Qcm]                     | O    | Qt6       |                            |
| [Telegram]                | O    | Qt6       |                            |
| [Electron-NCM]            | O    | Electron  |                            |
| [YesPlayMusic]            | O    | Electron  | R3Play会给标题加额外的后缀 |
| [youtube-music]           | O    | Electron  | 启用Shortcut               |
| [go-musicfox]             | O    | bubbletea | 4.3.2+                     |
| mpv                       | O    | 原生      | [mpv-mpris]                |

## 无法使用的播放器

[listen1-desktop]: https://github.com/listen1/listen1_desktop

| Player            | OSD | issue       | 备注 |
| ----------------- | --- | ----------- | ---- |
| qqmusic           | O   | 位置一直为0 |      |
| [listen1-desktop] | O   | 位置一直为0 |      |

## 目录

一般情况会创建的目录（可能被用户XDG设置影响）

### Linux/Unix

```
~/.cache/waylyrics/XX/...
~/.config/waylyrics/...
# waylyrics 会首先尝试在这里加载主题，找不到的话就从全局模板目录找
~/.local/share/waylyrics/_themes/...
```

### Windows

```
%AppData%\poly000\waylyrics\config
%AppData%\poly000\waylyrics\data
%LocalAppData%\poly000\waylyrics\cache
```

## 替代品

[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics
[Sunamu]: https://github.com/NyaomiDEV/Sunamu
[lyricsSeeker]: https://github.com/BruceZhang1993/LyricsSeeker
[caraoke-plasmoid]: https://github.com/Copay/caraoke-plasmoid
[desktop-lyric]: https://github.com/tuberry/desktop-lyric
[可道歌词]: https://www.autolyric.com/
[Lyricify]: https://github.com/WXRIW/Lyricify-App
[osdlyrics]: https://github.com/osdlyrics/osdlyrics

### Linux

| 名称                          | 技术栈        | 桌面支持      | 播放器支持 | 备注 |
| ----------------------------- | ------------- | ------------- | ---------- | ---- |
| [waybar-netease-music-lyrics] | bash          | wlroots-based | NCM[^0]    |      |
| [Sunamu]                      | Electron/TS   | X/Xwayland    | MPRIS      |      |
| [desktop-lyric]               | JavaScript    | GNOME         | MPRIS      |      |
| [caraoke-plasmoid]            | QML           | Plasma        | MPRIS      |      |
| [osdlyrics]                   | GTK2/C+Python | X/Xwayland    | MPRIS      |      |
| [lyricsSeeker]                | Python+QML    | Plasma        |            | WIP  |

[^0]: https://github.com/kangxiaoju/waybar-netease-music-lyrics/blob/f84810fe1628ca38fa36d88506152d88cf0e77e4/song.sh#L41-L59

### Windows

| 名称       | 技术栈               | 播放器支持                                                                                          | 备注 |
| ---------- | -------------------- | --------------------------------------------------------------------------------------------------- | ---- |
| [可道歌词] | C++                  | Windows Media Player<br>Foobar2000<br>AIMP<br>Winamp                                                |      |
| [Lyricify] | UWP/C#(4+)<br>C#(3-) | Spotify<br>iTunes<br>Apple Music<br>Music Center<br>QQ Music<br>Netease Cloud Music<br>YesPlayMusic |      |

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
