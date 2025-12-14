# Waylyrics

<div align="center">
  <img src="img/waylyrics.png" style="height: 200px; width: auto;"></img>
  <br />
  <div>
    <a href="https://t.me/+FWgnE0GRDYZhNjc1"><img src="https://img.shields.io/badge/Join-Telegram-blue.svg?logo=telegram"></a>
    <a href="https://matrix.to/#/#waylyrics_x:catgirl.cloud"><img src="https://img.shields.io/badge/Join-Matrix-blue.svg?logo=matrix"></a>
    <br />
    <a href="https://www.bilibili.com/video/av1555055010/" target="blank"><img src="https://img.shields.io/badge/bilibili-Preview-blue.svg?logo=bilibili"></a>
  </div>
</div>

[简体中文](./README.md) | [English](./README.en.md)

- [Introduction](#introduction)
- [Preview](#preview)
- [Build/Install](#buildinstall)
- [Usage](#usage)
- [Requirement](#requirement)
- [Plugins](#plugins)
- [Recommended Players](#recommended-players)
- [Unsupported Players](#unsupported-players)
- [Directories](#directories)
- [Alternatives](#alternatives)
- [Credit](#credit)
- [License](#license)


## Introduction

Waylyrics is a desktop lyrics display application that supports both Linux and Windows.

Key Features:
- Built with GTK 4
- Maintains up-to-date dependencies
- Automatic light/dark theme switching
- Uses TOML format for configuration files
- Automatically adds latest comments to configuration files on startup
- Highly customizable CSS themes with [multiple presets available](themes)
- Compatible with any player that properly implements MPRIS/SMTC protocol
- Continuously improved lyrics integration with various music players through community contributions

## Preview

### Main UI

> You can hide the decoration by `Ctrl-D` or `Hide Decoration` in tray icon menu.

![Main UI](img/en-cn.png)

### Search Lyrics

![Search Lyrics](img/search-window.jpg)


## Build/Install

> MSRV: 1.78.0

Check [INSTALLATION.en.md](doc/INSTALLATION.en.md)

## Usage

Also check [desktop file](metainfo/io.github.waylyrics.Waylyrics.desktop)

https://github.com/waylyrics/waylyrics/blob/d2132b42b135e0de09640de6a5a0b4797871c67e/src/config.rs#L67-L71

## Requirement

- A player at least supports MPRIS PlaybackStatus, Position and Metadata(with title)
- In particular, a wm allows you set windows as top-level

## Plugins

[waylyrics-sakura-translator]: https://github.com/WithourAI/waylyrics-sakura-translator
[SakuraLLM]: https://github.com/SakuraLLM/Sakura-13B-Galgame

| Plugin                        | Description                             | Version |
| ----------------------------- | --------------------------------------- | ------- |
| [waylyrics-sakura-translator] | ja->zh translator based on [SakuraLLM ] | v0.3.6+ |

## Recommended Players

### Linux

[extension]: https://addons.mozilla.org/en-US/firefox/addon/plasma-integration/
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

[mpv-mpris]: https://github.com/hoyon/mpv-mpris

[mpd-mpris]: https://github.com/natsukagami/mpd-mpris
[ncmpcpp]: https://github.com/ncmpcpp/ncmpcpp

[DeaDBeeF]: https://deadbeef.sourceforge.io/
[deadbeef-mpris2-plugin]: https://github.com/DeaDBeeF-Player/deadbeef-mpris2-plugin

[SPlayer]: https://github.com/imsyy/SPlayer

[listen1-desktop]: https://github.com/listen1/listen1_desktop
[listen1-pr]: https://github.com/listen1/listen1_desktop/pull/1325

| Name                      | Online? | Framework | #                                          |
| ------------------------- | ------- | --------- | ------------------------------------------ |
| Firefox                   | O       | Browser   | needs [extension]                          |
| Chrome                    | O       | Browser   |                                            |
| [DeaDBeeF]                | X       | GTK3      | [deadbeef-mpris2-plugin]                   |
| [netease-cloud-music-gtk] | O       | GTK4      | 2.3.0+                                     |
| [amberol]                 | X       | GTK4      |                                            |
| [lollypop]                | X       | GTK4      |                                            |
| [FeelUOwn]                | O       | Qt5       | 3.9.12+                                    |
| [VLC]                     | O       | Qt5       |                                            |
| [Qcm]                     | O       | Qt6       |                                            |
| [Telegram]                | O       | Qt6       |                                            |
| [listen1-desktop]         | O       | Electron  | v2.33.0 [修复][listen2-pr]                 |
| [Electron-NCM]            | O       | Electron  |                                            |
| [YesPlayMusic]            | O       | Electron  | R3Play appends unexpected suffixs to title |
| [youtube-music]           | O       | Electron  | enable `Shortcut`                          |
| [go-musicfox]             | O       | bubbletea | 4.3.2+                                     |
| mpv                       | O       | Native    | load [mpv-mpris]                           |
| [ncmpcpp]                 | X       | ncursew   | [mpd-mpris]                                |

### Windows

| Name          | Online? | Framework | #                                                             |
| ------------- | ------- | --------- | ------------------------------------------------------------- |
| [FeelUOwn]    | O       | Qt5       | 4.0.1+                                                        |
| [SPlayer]     | O       | Electron  |                                                               |
| [go-musicfox] | O       | bubbletea | 4.4.0+                                                        |
| Media Player  | X       | UWP       | it will **NOT** update timeline <br> immediately after a seek |

## Unsupported Players


| Player  | OSD | issue               |
| ------- | --- | ------------------- |
| qqmusic | O   | position stays on 0 |

## Directories

### Linux/Unix

```
~/.cache/waylyrics/XX/...
~/.config/waylyrics/...
# waylyrics will first try load theme here, if not exists, load from global template.
~/.local/share/waylyrics/_themes/...
```

### Windows

```
%AppData%\poly000\waylyrics\config
%AppData%\poly000\waylyrics\data
%LocalAppData%\poly000\waylyrics\cache
```

## Alternatives

[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics
[Sunamu]: https://github.com/NyaomiDEV/Sunamu
[lyricsSeeker]: https://github.com/BruceZhang1993/LyricsSeeker
[caraoke-plasmoid]: https://github.com/Copay/caraoke-plasmoid
[desktop-lyric]: https://github.com/tuberry/desktop-lyric
[AutoLyric]: https://www.autolyric.com/
[Lyricify]: https://github.com/WXRIW/Lyricify-App
[osdlyrics]: https://github.com/osdlyrics/osdlyrics
[desktop_lyric]: https://github.com/Moeweb647252/desktop_lyric
[lyrica]: https://github.com/chiyuki0325/lyrica
[lyric-for-musicfox]: https://github.com/SmileYik/lyric-for-musicfox

### Linux

| Name                          | Stack         | DE/WM         | Player Support     | #   |
| ----------------------------- | ------------- | ------------- | ------------------ | --- |
| [lyric-for-musicfox]          | Qt/C++        | *             | musicfox           |     |
| [waybar-netease-music-lyrics] | bash          | wlroots-based | Any NCM Player[^0] |     |
| [Sunamu]                      | Electron/TS   | X/Xwayland    | MPRIS              |     |
| [desktop-lyric]               | JavaScript    | GNOME         | MPRIS              |     |
| [caraoke-plasmoid]            | QML           | Plasma        | MPRIS              |     |
| [osdlyrics]                   | GTK2/C+Python | X/Xwayland    | MPRIS              |     |
| [lyricsSeeker]                | Python+QML    | Plasma        |                    | WIP |
| [desktop_lyric]               | egui          | Plasma        | MPRIS              |     |
| [lyrica]                      | Rust+QML      | Plasma        | MPRIS              |     |

[^0]: https://github.com/kangxiaoju/waybar-netease-music-lyrics/blob/f84810fe1628ca38fa36d88506152d88cf0e77e4/song.sh#L41-L59

### Windows

| Name        | Stack                | Player Support                                                                                      | #   |
| ----------- | -------------------- | --------------------------------------------------------------------------------------------------- | --- |
| [AutoLyric] | C++                  | Windows Media Player<br>Foobar2000<br>AIMP<br>Winamp                                                |     |
| [Lyricify]  | UWP/C#(4+)<br>C#(3-) | Spotify<br>iTunes<br>Apple Music<br>Music Center<br>QQ Music<br>Netease Cloud Music<br>YesPlayMusic |     |
| [HotLyric]  | WinUI3               | HyPlayer<br>LyricEase<br>YesPlayMusic<br>*GSMTC*s[^1]                                               |     |

[HotLyric]: https://github.com/cnbluefire/HotLyric
[^1]: https://github.com/cnbluefire/HotLyric?tab=readme-ov-file#%E6%94%AF%E6%8C%81%E7%9A%84%E6%92%AD%E6%94%BE%E5%99%A8

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
