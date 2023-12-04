# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/+FWgnE0GRDYZhNjc1)
[![matrix-group](https://img.shields.io/matrix/waylyrics_x:catgirl.cloud.svg?server_fqdn=matrix.catgirl.cloud)](https://matrix.to/#/#waylyrics_x:catgirl.cloud)

Simple universal on screen lyrics made with GTK4 and ❤️.

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
      - [local](#local)
        - [GTK+](#gtk-1)
        - [Native](#native)
        - [Qt](#qt-1)
    - [MPRIS-unfriendly Players](#mpris-unfriendly-players)
  - [Directories](#directories)
  - [Approach](#approach)
  - [Alternatives](#alternatives)
  - [Credit](#credit)
  - [License](#license)

## Build/Install

Check [INSTALLATION.en.md](INSTALLATION.en.md)

## Usage

Click the menu button at above right, there are numerous operations,
some with shortcuts.

SIGUSR1: disconnect from current player

SIGUSR2: switch gtk decoration on/off

## Requirement

- A player supports MPRIS well, especially should made TrackID unique between songs
- In particular, a wm allows you set windows as top-level one's.

### Recommended Players

#### online

> For Qcm, Feeluown-{netease, qqmusic}, ElectronNCM, YesPlayMusic, we could get song id directly

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
- [youtube-music](https://github.com/th-ch/youtube-music), has lots of songs and MV support


#### local

##### GTK+

- [amberol](https://gitlab.gnome.org/World/amberol)

##### Native

- mpv + [mpv-mpris](https://github.com/hoyon/mpv-mpris)

##### Qt

- [VLC](https://www.videolan.org)

### MPRIS-unfriendly Players

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

## Approach

Current approach my seems dirty:

1. get the likely actived player when none is connnected, and disconnect from one only if it's not avaliable more
2. keep sync with 2s interval and 20ms refresh for lyrics
3. use the length-matched result (or first result if former is not found) of `search_song` and sync START in each run, fetch lyric only when needed

## Alternatives

[YesPlayMusicOSD]: https://github.com/shih-liang/YesPlayMusicOSD
[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics
[lx-music-desktop]: https://github.com/lyswhut/lx-music-desktop
[Sunamu]: https://github.com/NyaomiDEV/Sunamu
[lyricsSeeker]: https://github.com/BruceZhang1993/LyricsSeeker
[caraoke-plasmoid]: https://github.com/Copay/caraoke-plasmoid
[desktop-lyric]: https://github.com/tuberry/desktop-lyric

For Sway users, you may want use [waybar-netease-music-lyrics].

[YesPlayMusicOSD] and [lx-music-desktop] has great lyrics support

[Sunamu] could start web-server/launch electron app depends on your choice, and it has good-looking and it's much more feature rich than waylyrics.

BruceZhang1993's [lyricsSeeker] is still WIP, but it may have better-looking and better integration with KDE.

Copay's [caraoke-plasmoid] is currently Plasma-only, though it is easy to remove plasmoid components

If you are using GNOME I will suggest you [desktop-lyric].

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[The MIT License (MIT)](https://raw.githubusercontent.com/waylyrics/waylyrics/master/LICENSE)

This project icon is licensed under a [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
