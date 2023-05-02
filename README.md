# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/waylyrics)

Simple wayland native universal on screen lyrics.
Main logic runs single-thread, but tokio runs multi-threaded.

## Requirement

- A player supports MPRIS well.

### Recommended Players

- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [Qcm](https://github.com/hypengw/Qcm)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)

### MPRIS-unfriendly Players

[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[FeelUOwn]: https://github.com/feeluown/FeelUOwn


Player | OSD | issue
------|-----|------
[netease-cloud-music-gtk] | X | gives 0 position
[FeelUOwn] | O | gives 0 position and 0 length
Firefox | X | do not provide position call
qqmusic | O | no info other than title avaliable, but it's own OSD is pretty good

## Approach

Current approach my seems dirty:

1. get the likely actived player in each sync
2. keep sync with 3s interval and 100ms refresh for lyrics
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
