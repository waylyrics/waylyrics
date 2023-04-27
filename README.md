# Waylyrics

Simple wayland native universal on screen lyrics.

Current approach my seems dirty:

1. get the likely actived player when waylyrics starts
2. keep track with half intertal of lyric update's
3. use the first result of `search_song` and sync lyric until pause/next happens.

## Alternatives

[YesPlayMusicOSD]: https://github.com/shih-liang/YesPlayMusicOSD
[FeelUOwn]: https://github.com/feeluown/FeelUOwn
[doesn't support translation yet though]: https://github.com/feeluown/FeelUOwn/issues/643 
[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics

For Sway users, you may want use [waybar-netease-music-lyrics].

And for now, [YesPlayMusicOSD]
and [FeelUOwn]
are both of great lyrics support, latter [doesn't support translation yet though].

BruceZhang1993's [lyricsSeeker](https://github.com/BruceZhang1993/LyricsSeeker) is still WIP, but it may have better-looking and better integration with KDE.

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.
