# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/waylyrics)

Simple universal on screen lyrics made with GTK4 and ❤️.

![](https://user-images.githubusercontent.com/34085039/235869618-1f0fe78d-2637-4898-b8a1-53eb015d6731.png)

## Build/Install

Check [INSTALLATION.md](INSTALLATION.md)

## Requirement

- A player supports MPRIS well.
- In particular, a wm allows you set windows as top-level one's.

### Recommended Players

online:
- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [Qcm](https://github.com/hypengw/Qcm)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)
- [Telegram](https://t.me/Music163Bot)
- [FeelUOwn](https://github.com/feeluown/FeelUOwn/), after the [Pull Request](https://github.com/feeluown/FeelUOwn/pull/690) merged.

local:
- [mpv-mpris](https://github.com/hoyon/mpv-mpris)
- [VLC](https://www.videolan.org)

### MPRIS-unfriendly Players

[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[flutter-netease-music]: https://github.com/boyan01/flutter-netease-music


Player | OSD | issue
------|-----|------
[netease-cloud-music-gtk] | X | gives 0 position
Firefox | X | do not provide position call
Chrom* | X | break the "unique" gurantee for TrackID
qqmusic | O | no info other than title/artist avaliable
[flutter-netease-music] | X | no mpris support

## Approach

Current approach my seems dirty:

1. get the likely actived player when none is connnected, and disconnect from one only if it's not avaliable more
2. keep sync with 2s interval and 20ms refresh for lyrics
3. use the length-matched result (or first result if former is not found) of `search_song` and sync START in each run, fetch lyric only when needed

## Alternatives

[YesPlayMusicOSD]: https://github.com/shih-liang/YesPlayMusicOSD
[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics

For Sway users, you may want use [waybar-netease-music-lyrics].

[YesPlayMusicOSD] have great lyrics support

BruceZhang1993's [lyricsSeeker](https://github.com/BruceZhang1993/LyricsSeeker) is still WIP, but it may have better-looking and better integration with KDE.

Copay's [caraoke-plasmoid](https://github.com/Copay/caraoke-plasmoid) is currently Plasma-only, though it is easy to remove plasmoid components

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.
