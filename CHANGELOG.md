# Changelog

All notable changes to this project will be documented in this file.

## [0.2.11] - 2024-02-22

### Bug Fixes

- Restart in tray-icon set click-through
- Use hsla for transparent window background
- Hsl color format

### Documentation

- Update changelog
- Update changelog
- Remove dead `flutter-netease-music`

### Miscellaneous Tasks

- Disable changelog on tag due to shitty failure [ci skip]
- Bump dependencies
- Release v0.2.11

### Performance

- Store default MainContext

### Refactor

- Use `clone!` macro rather than calling upgrade/downgrade manually

### Enhance

- Switch-passthrough in tray-icon without restart

## [0.2.10] - 2024-02-21

### Documentation

- Update changelog

### Features

- Set lyric-display-mode in GSettings
- Split popover menu to UI section and Play section
- Set mouse click-through in GSettings
- New style
- (optional) select labels by translation/origin

### Miscellaneous Tasks

- Release v0.2.10

### Refactor

- Cleanup `set_lyric_with_mode` [ci skip]
- Move logic-related fields to Window::new

### Revert

- "feat: (optional) select labels by translation/origin"

### Styling

- Format CSS with Prettier and Stylelint

## [0.2.9] - 2024-02-21

### Documentation

- Update changelog

### Features

- Only show set-lyric button when avaliable
- Show icons on tray-icon menu
- [**breaking**] Set lyric align mode on run time

### Miscellaneous Tasks

- Fix changelog ci skipped unexpectedly
- Remove invalid github template
- Clean up theme comments
- Release v0.2.9

### I18n

- Translate length to 时长
- Translate display_mode
- Translate lyric align

## [0.2.8] - 2024-02-20

### Bug Fixes

- Changelog CI need write permission
- Incomplete i18n in menu

### Documentation

- Update changelog
- Update changelog
- Update changelog
- Update readme
- Update changelog
- Update changelog
- Add gettext to dependencies

### Features

- Sort search result by fuzzy-match weight
- Show tooltip for search entries
- I18n support

### Miscellaneous Tasks

- Generate changelogs for master
- Update changelog on each commit
- Do not flood commit logs
- Remove unnessacary use of Arc<dyn LyricProvider>
- Release v0.2.8

### Refactor

- Skip fuzzy-match in it's block

### Styling

- Package_name as a constant [ci-skip]

### Build

- Add icon
- Make global theme presets optional
- Package with i18n files

### I18n

- Add translation for Simplefied Chinese

## [0.2.7] - 2024-02-19

### Features

- Restart waylyrics in tray-icon
- Fuzzy-match lyrics with Sorensen-Dice coefficient
- Skip fuzzy-match if we got only title in metadata

### Miscellaneous Tasks

- Migrate to better dice-coefficient lib
- Release v0.2.7

### Enhance

- Remove unnessacary global variable for PlayerId's

### Log

- Log detailed likelihood with trace-level

## [0.2.6] - 2024-02-18

### Bug Fixes

- Missing feature gate for re-export in sync

### Features

- Set lyric display mode in run time
- Initial tray-icon support
- PlayAction control
- Optionally start tray-icon service

### Miscellaneous Tasks

- Release v0.2.6

### Refactor

- Remove repeated function prefix

### Todo

- Impl real tray-icon functions

## [0.2.5] - 2024-02-18

### Bug Fixes

- Do not hide below label in set_lyric
- Remove `hide_label_on_empty_text = false` support
- Do not hide below label in set_lyric
- Remove `hide_label_on_empty_text = false` support
- Append comments with `documented`
- Repeated docs appends to commented config.toml

### Documentation

- Lyric-align accepts CamelCase value

### Features

- Generate comments for config

### Miscellaneous Tasks

- Release v0.2.5

### Refactor

- Move search_window to crate::app
- Move search_window to crate::app
- Move config.rs to config/

### Build

- Add optional ksni dep

## [0.2.4] - 2024-02-17

### Bug Fixes

- Should re-export PlayAction
- Re-export play-action channel
- Do not show lyric on `Stopped` status

### Documentation

- Introduce Conventional Commit [ci skip]
- Add logo and intro video [ci skip]
- Move chat banners to center block [ci skip]
- Waybar supports all wlroots-based compositors

### Features

- Invoke app actions with mpsc::channel
- Invoke ui actions with mpsc::channel
- Set unsupported reason on below label
- Hide default text on idle

### Miscellaneous Tasks

- Make logo not so shining in dark background [ci skip]
- Migrate to gtk4-rs 0.8.0
- Release 0.2.4

### Refactor

- Rename AppAction to PlayAction
- Move UI-related actions to crate::app::actions
- Rename to play_action

### Styling

- Remove unused Downgrade import

### Build

- Make tray-icon feature optional
- Rename feature to 'action-event' rather than tray-icon

### Log

- Log reveiced action event

### Other

- White shade for text and logo

## [0.2.3] - 2024-02-13

### Bug Fixes

- Set 20ms as default lyric update interval

### Documentation

- Play with musicfox need position fix patch
- Add `osdlyrics` to alternatives [ci skip]
- Explain more fields in `Config`
- Firefox via Plasma Integration [ci skip]
- `lollypop`, GTK3-based local music player
- Restate outdated doc of `TrackMeta`

### Features

- Intro `is_likely_songid` for songid verification
- Set empty label explictly
- Allow to show origin lyric in above
- Add `trans` theme [ci skip]

### Miscellaneous Tasks

- Remove unreachable `title.unwrap_or()` call
- Migrate to dtolnay/rust-toolchain
- Release 0.2.3

### Refactor

- Rename confusing `match_lyric` to `verify_lyric` [ci skip]
- Apply clippy fix [ci skip]

### Testing

- Add unit tests for QQMusic::init

## [0.2.2] - 2024-02-10

### Bug Fixes

- QQMusic::init should panic on `Err()` rather than `Ok()`

### Miscellaneous Tasks

- Test qqmusic provider initializing
- Release `v0.2.2`

### Build

- Add ability to disable tests require network

## [0.2.1] - 2024-02-08

### Documentation

- Define `lrc_iter` behaviour
- Explain build environment variable
- Download pre-built executables [ci skip]

### Miscellaneous Tasks

- Remove unused import
- Run real test in CI
- Release 0.2.1

### Refactor

- Make `init_dirs` a public method so we could write tests
- Remove `CONFIG_HOME`
- Impl into_owned for LyricLine
- Setup `QQMusic` with `init()` call
- Rename `get_lyric` to `parse_lyric` [ci skip]

### Testing

- Test netease lyric get & parse
- Test LRC parsing
- Move unit tests to inside `src/`
- Move out doctest for `get_lrc_path`

### Breaking

- Drop `SIGUSR` control support

### Build

- Improve test build time

## [0.2.0] - 2024-02-08

### Bug Fixes

- Correct variable name typo in lib.rs
- Better Gtk CSD UI
- Clickthrough not working with decoration
- Cannot restore decoration after set invisable
- Gen_config_example was removed
- Translation lyric not showing
- OpenSUSE packaging
- Update smoketest.yml
- Lyrics are not trimed
- Do not try build doc on PR
- Get songmid for lyric query
- XML decode for LRC from QQMusic api
- Set serde default for Triggers
- Return Ok when fetch successfully
- Refetch lyric should ignore cache
- RPM distros packaging
- Add white background
- Set default on empty lyric status
- Deepsource: anti-pattern use of
- Recorrect player_name part for NCM-gtk4
- Missing white background
- Missing white color
- Make cargo pass dbus check by introducing dbus-dummy
- LyricHint::File from mpris should decode with `url::Url::to_file_path`
- Unconfigured provider from hint will cause lyrics not to be loaded
- Set default lyric_update interval to 50ms
- Skip UTF-8 BOM so lrc-nom will work
- Cannot return value referencing function parameter
- Desktop file does not need a launch action [ci skip]
- Ignore invalid LRC lines
- Override user theme
- Hint support for musicfox

### Documentation

- Fix typo and add schema compilation
- Explain triggers fields
- Mention default shortcuts
- Update outdated requirement
- Note to install pango
- Remove `lx-music` because it's dead [ci skip]

### Features

- Lyric play
- Custom font family
- Sync with mpris2
- Allow auto hide empty label & code refactor
- #8
- Rpm spec (#31)
- Disconnect from current player if received SIGUSR1, fix #43
- Switch decoration on SIGUSR2, fix #49
- Add new fields to old configuration
- Allow to build libdbus-sys vendored
- Change project icon
- Place menubutton at end
- Switch passthrough
- Switch decoration with shortcut
- Switch click-through with Alt-P
- Reload theme
- Reload lyric (from cache)
- Change project icon
- Change project icon
- Query_lyric for QQMusic
- Init api client
- Init multi provider support
- Weighted multi-source lyric search
- Change project icon
- Change project icon
- Change project icon
- Search songs from QQ音乐
- Append Reload lyric to CSD menu
- Refetch lyric (ignore cache)
- Mimalloc as default allocator
- Support players do not provide TrackID
- Song_id trick for NCM-gtk
- New design
- Try load lyric from disk
- Metadata from LyricHint (for music_file without local lyrics)
- Improve RPM group
- Add support for musicfox

### Miscellaneous Tasks

- Migrate to anyhow::Result
- Add smoketest
- Rust caching and weston
- Configurarion for better performance
- Log track_id
- Use default value for missing config fields
- Log PID
- Update dependencies weekly
- Update doc
- Fix broken group link
- Telegram group
- Update doc
- Log cache_path with info level
- Keep example env clear
- Update doc
- Remove useless declaration
- Fix default configuration
- Redesign icon
- 🎵 instead of 🎶
- Build with nightly toolchain
- Remove some unreachable code
- Remove some unreachable code
- Use Cell for T: Copy
- Typing for TrackState
- `extract connect_factory`
- Add debug msg for config and theme path
- Also log result weight
- Asynchronously fetch lyric
- Replace `RefCell` with `Cell`
- Add debug msg for get_cache_path
- Specify revision in Cargo.toml for repreducablily
- Do not encrypt heap allocations
- Add dhat.out.* to .gitignore
- Add mimalloc to dependencies
- Avoid hard coding a must be same value twice
- Log selected song id
- Introduce 洛雪
- Add .deepsource.toml
- Update .deepsource.toml
- Logging for future spawn
- Update ncm-gtk to upstream
- Remove some outdated log
- Correct legacy field name
- Code cleanup
- Code cleanup
- Apply lint const_block
- Cargo BuildRequires for Fedora
- Use sort_by_key
- Refactor doc directory [ci skip]
- Update description [ci skip]
- Intro openssl/vendored to vendored feature [ci skip]
- Mention desktop file for global shortcut
- Replace `with_borrow_mut` with `set` [ci skip]
- Clean up lyric scroll
- Update alternatives [ci skip]
- Update translation [ci skip]
- Drop support for ncm-gtk legacy name
- Update doc for youtube-music [ci skip]
- Add `musicfox`, a TUI based music player [ci skip]
- Release 0.2.0

### Refactor

- Sync::player
- Interop/lyric-fetch/lyric-scroll
- Replace unneeded field pattern with `..`
- Setup helpers
- Move out update_lyric

### Styling

- Use RPM macros to replace hard-coded directories
- Define `glib-macros` under `gtk`

### Testing

- Add test for `get_lyric_path`
- Fix get_lrc_path doctesr

### Build

- Let install to make directory
- Package LICENSE and README
- Fix install

### Config

- Label alignment

### Enhance

- Adjust layout about switch player
- Direct id support for YesPlayMusic
- Do not copy template themes
- Ship with a generated template is useless
- Save GTK+ CSD state
- Trim lyrics on set
- Set Align::Start for column entries
- Better layout for search_window
- Log to journalctld
- Distingush label for whether lyrics were cached
- Apply Mutex<()> lock for update_lyrics
- Album+title for qqmusic
- Handle -1901 error

### Hotfix

- Disable QQMusic source by default
- Fix mpris player connect

### Log

- Info - player hint
- Log errors on local lyric loading

### Merge

- Remote/master
- Switch-click-through-shortcut

### Search

- Set resizeable for title, singer and album

### Todo

- Query play status and sync on update
- Fix not fetching lyrics
- Impl search & fetch_lyric
- Use an external build system

<!-- generated by git-cliff -->
