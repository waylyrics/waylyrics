# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/+FWgnE0GRDYZhNjc1)
[![matrix-group](https://img.shields.io/matrix/waylyrics_x:catgirl.cloud.svg?server_fqdn=matrix.catgirl.cloud)](https://matrix.to/#/#waylyrics_x:catgirl.cloud)

Simple desktop lyrics made with GTK4 and ❤️.

![](https://github.com/waylyrics/waylyrics/assets/34085039/dd7d9236-b2ae-47da-b4a3-e19a7d10e31b)

- [Waylyrics](#waylyrics)
  - [Build/Install](#buildinstall)
  - [Usage](#usage)
  - [Requirement](#requirement)
    - [Recommended Players](#recommended-players)
      - [online](#online)
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

SIGUSR1: disconnect from current player

SIGUSR2: switch gtk decoration on/off

## Requirement

- A player at least supports MPRIS PlaybackStatus, Position and Metadata(with title)
- In particular, a wm allows you set windows as top-level

### Recommended Players

#### online

> For Qcm, Feeluown-{netease, qqmusic}, ElectronNCM, YesPlayMusic(OSD), we could get song id directly

##### GTK+

- [netease-cloud-music-gtk](https://github.com/gmg137/netease-cloud-music-gtk)

##### Qt

- [Qcm](https://github.com/hypengw/Qcm)
- [FeelUOwn](https://github.com/feeluown/FeelUOwn/), with the latest code
- [Telegram](https://t.me/Music163Bot)

##### Electron

- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [lx-music-desktop](https://github.com/lyswhut/lx-music-desktop)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)
- [youtube-music](https://github.com/th-ch/youtube-music), enable ShortCut plugin

##### TUI

- [go-musicfox](https://github.com/go-musicfox/go-musicfox)

#### local

##### GTK+

- [amberol](https://gitlab.gnome.org/World/amberol)

##### Native

- mpv + [mpv-mpris](https://github.com/hoyon/mpv-mpris)

##### Qt

- [VLC](https://www.videolan.org)

### Players with bad MPRIS support

[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[flutter-netease-music]: https://github.com/boyan01/flutter-netease-music
[youtube-music]: https://github.com/th-ch/youtube-music


| Player                  | OSD | issue                        |
| ----------------------- | --- | ---------------------------- |
| Firefox                 | X   | do not provide position call |
| qqmusic                 | O   | position stays on 0          |
| [flutter-netease-music] | X   | no mpris support             |

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

### Linux

| Name                          | Stack       | DE/WM      | Player Support                     | #   |
| ----------------------------- | ----------- | ---------- | ---------------------------------- | --- |
| [waybar-netease-music-lyrics] | bash        | Sway       | Any NetEase Cloud Music Player[^0] |     |
| [Sunamu]                      | Electron/TS | X/Xwayland | MPRIS                              |     |
| [desktop-lyric]               | JavaScript  | GNOME      | MPRIS                              |     |
| [lyricsSeeker]                | Python+QML  | Plasma     |                                    | WIP |
| [caraoke-plasmoid]            | QML         | Plasma     | MPRIS                              |     |

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
