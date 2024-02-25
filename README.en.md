# Waylyrics

<p align="center">
  <img src="img/waylyrics.png" style="height: 30vw"></img>
  <br />
  <a href="https://t.me/+FWgnE0GRDYZhNjc1">Telegram</a>&nbsp;|&nbsp;
  <a href="https://matrix.to/#/#waylyrics_x:catgirl.cloud">Matrix</a>
  <br />
  <a href="https://www.bilibili.com/video/BV1ap421R7nD" target="blank">Feature Preview</a>
</p>

- [Waylyrics](#waylyrics)
  - [Build/Install](#buildinstall)
  - [Usage](#usage)
  - [Requirement](#requirement)
  - [Recommended Players](#recommended-players)
    - [online](#online)
      - [Browser](#browser)
      - [GTK+](#gtk)
      - [Qt](#qt)
      - [Electron](#electron)
      - [TUI](#tui)
    - [local](#local)
      - [GTK+](#gtk-1)
      - [Native](#native)
      - [Qt](#qt-1)
  - [Players with bad MPRIS support](#players-with-bad-mpris-support)
  - [Directories](#directories)
  - [Alternatives](#alternatives)
    - [Linux](#linux)
    - [Windows](#windows)
  - [Credit](#credit)
  - [License](#license)

## Build/Install

Check [INSTALLATION.en.md](doc/INSTALLATION.en.md)

## Usage

Also check [desktop file](io.poly000.waylyrics.desktop)

https://github.com/waylyrics/waylyrics/blob/d2132b42b135e0de09640de6a5a0b4797871c67e/src/config.rs#L67-L71

## Requirement

- A player at least supports MPRIS PlaybackStatus, Position and Metadata(with title)
- In particular, a wm allows you set windows as top-level

## Recommended Players

### online

#### Browser

[plasma integration]: https://addons.mozilla.org/en-US/firefox/addon/plasma-integration/

- Chrome
- Firefox, with [plasma integration]

#### GTK+

- [netease-cloud-music-gtk](https://github.com/gmg137/netease-cloud-music-gtk)

#### Qt

- [Qcm](https://github.com/hypengw/Qcm)
- [FeelUOwn](https://github.com/feeluown/FeelUOwn/), with the latest code
- [Telegram](https://t.me/Music163Bot)

#### Electron

- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)
- [youtube-music](https://github.com/th-ch/youtube-music), enable ShortCut plugin

#### TUI

- [go-musicfox](https://github.com/go-musicfox/go-musicfox), after [fb7e486](https://github.com/go-musicfox/go-musicfox/commit/fb7e4865a39c9537f3868d62dae7c8690a9ca8c4)

### local

#### GTK+

- [amberol](https://gitlab.gnome.org/World/amberol)
- [lollypop](https://github.com/hamonikr/lollypop)

#### Native

- mpv + [mpv-mpris](https://github.com/hoyon/mpv-mpris)

#### Qt

- [VLC](https://www.videolan.org)

## Players with bad MPRIS support

| Player  | OSD | issue               |
| ------- | --- | ------------------- |
| qqmusic | O   | position stays on 0 |

## Directories

Typically,

```
~/.cache/waylyrics/XX/...
~/.config/waylyrics/...
# waylyrics will first try load theme here, if not exists, load from global template.
~/.local/share/waylyrics/_themes/...
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

### Linux

| Name                          | Stack         | DE/WM         | Player Support     | #   |
| ----------------------------- | ------------- | ------------- | ------------------ | --- |
| [waybar-netease-music-lyrics] | bash          | wlroots-based | Any NCM Player[^0] |     |
| [Sunamu]                      | Electron/TS   | X/Xwayland    | MPRIS              |     |
| [desktop-lyric]               | JavaScript    | GNOME         | MPRIS              |     |
| [caraoke-plasmoid]            | QML           | Plasma        | MPRIS              |     |
| [osdlyrics]                   | GTK2/C+Python | X/Xwayland    | MPRIS              |     |
| [lyricsSeeker]                | Python+QML    | Plasma        |                    | WIP |

[^0]: https://github.com/kangxiaoju/waybar-netease-music-lyrics/blob/f84810fe1628ca38fa36d88506152d88cf0e77e4/song.sh#L41-L59

### Windows

| Name        | Stack                | Player Support                                                                                      | #   |
| ----------- | -------------------- | --------------------------------------------------------------------------------------------------- | --- |
| [AutoLyric] | Unknown              | Windows Media Player<br>Foobar2000<br>AIMP<br>Winamp                                                |     |
| [Lyricify]  | UWP/C#(4+)<br>C#(3-) | Spotify<br>iTunes<br>Apple Music<br>Music Center<br>QQ Music<br>Netease Cloud Music<br>YesPlayMusic |     |

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
