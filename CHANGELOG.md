## [0.3.22] - 2026-05-25

### 🚀 Features

- Optionally auto connect a player
- Add GAction for manual label set
- Listen to `SetAboveLabel` & `SetBelowLabel` signals
- Gh-346, support custom config path
- Skip auto-search for new track
- Support LyricX-like style

### 🐛 Bug Fixes

- *(windows)* Build script
- Only clean labels on reconnection
- Actually register win.set-label action
- Panic on empty `lyric-search-source` in manual searching
- *(ci)* Should run test on `Cargo.toml` update
- Migrate windows code
- New-style subscribe API requires to hold returned reference
- SetBelowLabel was not handled correctly
- Fill, center, left, right alignment
- Follow breaking change from lotfy
- Alignment of multi-line text in single label
- Get correct MPRIS player name part

### 💼 Other

- *(deps)* Bump dependabot/fetch-metadata from 2.3.0 to 2.4.0
- *(deps)* Bump actions/checkout from 4 to 5
- *(deps)* Bump actions/upload-artifact from 4 to 5
- *(deps)* Bump actions/checkout from 5 to 6
- Translate the new configuration tip
- Ai translation for italiano
- Add italiano translation by @albanobattistella
- *(deps)* Bump actions/upload-artifact from 5 to 6
- Replace once_cell with std::sync::LazyLock
- Detailed logs for signals debugging
- *(deps)* Bump dependabot/fetch-metadata from 2.4.0 to 2.5.0
- *(deps)* Bump actions/upload-artifact from 6 to 7
- *(deps)* Bump dependabot/fetch-metadata from 2.5.0 to 3.0.0
- *(deps)* Bump dependabot/fetch-metadata from 3.0.0 to 3.1.0

### 📚 Documentation

- Listen1 released with fix
- Note on readme translation
- *(readme)* Fix broken hyperlink

### 🎨 Styling

- [ci skip] rename arg to args
- Prefer .cloned() [ci skip]
- Reorder imports
- Force format style to `imports_granularity = "Module"`
- Fix DeepSource lint

### ⚙️ Miscellaneous Tasks

- Remove unused imports
- Update metainfo for flathub
- Automatically merge CI passed dependency bump
- *(dependabot)* Group toml* dependencies
- Fix release date of 0.3.21
- Migration to gtk4 0.10
- Only run test & docs CI when needed
- Fix auto merge CI
- Use the latest release action
- Skip online tests in CI
- Fix typo
- Migrate to latest ksni::TrayIcon api style
- Don't trigger CI twice time for PR
- Fix typo, translate comments, fix lints (#402)
- Update changelog for v0.3.22

### ◀️ Revert

- 8c55c88 new-style signal handler
- "revert: 8c55c88 new-style signal handler"
## [0.3.21] - 2025-04-13

### 🚀 Features

- 添加对从音乐文件元数据读取歌词的支持

### 🐛 Bug Fixes

- *(CI)* Remove outdated test of qqmusic api initialization [ci skip]
- 手动刷新歌词时清空歌词标签缓存
- .一个错误

### 💼 Other

- Migrate to ksni 0.3
- Switch to better tray-icon implementation for windows [ci skip]
- *(deps)* Bump dark-light to 2.0.0

### 📚 Documentation

- *(readme)* Replace text with logos
- *(readme)* Add preview images [ci skip]
- *(readme)* Add alternative `lyric-for-musixfox` [ci skip]
- *(readme)* Add introduction [ci skip]
- *(readme)* Note on project status
- *(readme)* HotLyric, alternative on windows [ci skip]
- Update changelog [ci skip]

### ⚡ Performance

- DashMap 作为歌词标签存在性缓存

### 🎨 Styling

- :lipstick: cargo fmt

### ⚙️ Miscellaneous Tasks

- Add .deepsource.toml
- Fix deepsource lints
- Dedup code
- Add script to export dup deps [ci skip]
## [0.3.20] - 2024-12-07

### 🐛 Bug Fixes

- Check if instance name used

### 💼 Other

- Capture panics
- Comment for sub table in config

### 🚜 Refactor

- Extract set_by_mode

### 📚 Documentation

- Update changelog & metainfo
## [0.3.19] - 2024-12-03

### 🚀 Features

- Implement color-scheme autoswitch (light/dark mode) (#277)
- Support multi-monitor setup (#278)

### 🐛 Bug Fixes

- LazyLock is not stable yet in 1.73 rustc
- Instance format

### 💼 Other

- Unregister latest theme
- Build on windows

### 📚 Documentation

- *(build)* Check MSRV
- Update metainfo & changelog

### ⚙️ Miscellaneous Tasks

- *(clippy)* Fix clippy warnings
## [0.3.18] - 2024-11-10

### 🚀 Features

- Set cookies for QQMusicApi in config
- Show errors from providers in search_window
- Support LRCLib provider

### 🐛 Bug Fixes

- *(win32)* Crash with Motrix runnig
- *(tray)* Export translated lyrics
- *(tray/unix)* Avoid calling list_players() from tray thread (#262)
- Set text wrap to show very-long lyric
- *(test)* Initialize QQMusic
- Add artists to QQMusic keyword
- Search failure dialog cannot be spawned outside GTK thread
- `login_qqmusic` call was not awaited

### 💼 Other

- Create Italian Translation  it_IT (#260)
- Init lyric provider with any struct

### 📚 Documentation

- *(readme)* Add alternative `lyrica`
- *(i18n)* Translation for contribution.md and build guide ubuntu (#272)
- Fix typo in filename

### 🧪 Testing

- Search/query via lrclib

### ⚙️ Miscellaneous Tasks

- Release v0.3.17
- Support nix flake (#264)
- Migrate to qqmusic-rs 0.2.0
- Apply clippy fix
- Bump version to 0.3.18
- Fix details URL in metainfo
## [0.3.16] - 2024-09-28

### 🚀 Features

- Show error dialog if search failed

### 🐛 Bug Fixes

- Build error on linux

### 💼 Other

- TrackMeta from Metadata will not fail now
- *(fix)* Fix build with MSVC
- Translate tips
- *(fix)* ""desktop

### 🚜 Refactor

- Dedup function signature

### 📚 Documentation

- *(readme)* Add alternative
- *(readme)* Intro SPlayer [ci skip]
- *(readme)* Desktop support of desktop_lyric, #258
- *(build)* Fix i18n part (#259)

### 🧪 Testing

- Add test for make_lrc_line

### ⚙️ Miscellaneous Tasks

- Release v0.3.16
## [0.3.15] - 2024-09-17

### 🐛 Bug Fixes

- Sec field in exported lyric

### 💼 Other

- Make import/export pormpt different
- Select output file after generation

### ⚙️ Miscellaneous Tasks

- Release v0.3.15
## [0.3.14] - 2024-09-17

### 🚀 Features

- Export-lyric

### 🐛 Bug Fixes

- Migrate to window-rs 0.58
- Support new type of HWND inner
- Build with `opencc` disabled
- Build with export-lyric/import-lyric disabled

### 💼 Other

- *(deps)* Bump orhun/git-cliff-action from 3 to 4 (#257)
- Dedup import-lyric
- Enable export-lyric for win build [ci skip]
- Update locale for 0.3.14

### 🚜 Refactor

- Migrate to gtk4-rs clone! new style
- Migrate to new-style clone!
- Prefer lazy-binding rather than multiple `cfg` flag
- Unify naming of same component in different mod
- Dedup import-*-lyric

### 📚 Documentation

- Mark issue/workaround for Better Comments
- *(readme)* Fix broken link for DeaDBeeF plugin [ci skip]

### ⚙️ Miscellaneous Tasks

- Disable unused warn
- Release v0.3.14
## [0.3.13] - 2024-06-27

### 🐛 Bug Fixes

- Do not remove original lyric on extracting translated ones
- Missing import statement

### 📚 Documentation

- *(readme)* Add `ncmpcpp` via mpd-mpris

### ⚙️ Miscellaneous Tasks

- Release v0.3.13
## [0.3.12] - 2024-06-20

### 🐛 Bug Fixes

- `extract-translated-lyric` flag is ignored
- Extract lyric lines should compare start_time

### 📚 Documentation

- Fix typo

### ⚙️ Miscellaneous Tasks

- *(log)* Add log for textdomain to use
- Release v0.3.12
## [0.3.11] - 2024-06-06

### 📚 Documentation

- *(install)* User-fiendly version (#240)

### ⚙️ Miscellaneous Tasks

- Release v0.3.11
## [0.3.10] - 2024-05-30

### 🐛 Bug Fixes

- *(log)* I18n bind result will not be logged

### ⚙️ Miscellaneous Tasks

- Release v0.3.10
## [0.3.9] - 2024-05-26

### 🐛 Bug Fixes

- *(test)* Do not test time
- *(windows)* Auto connect to active player

### 💼 Other

- Workaround for musicfox v4.4.0 [ci skip]
- Enable import lyric feature for windows

### 📚 Documentation

- *(readme)* Intro musicfox, TUI music player supports SMTC [ci skip]

### ⚙️ Miscellaneous Tasks

- Release v0.3.9
## [0.3.8] - 2024-04-29

### 🚀 Features

- Import local lrc in tray & csd menu

### 💼 Other

- Translate 'import lyric' & translated/original lyric

### 📚 Documentation

- *(readme)* Intro waylyrics-sakura-translator [ci skip]

### ⚙️ Miscellaneous Tasks

- Release v0.3.8
## [0.3.7] - 2024-04-29

### 🚀 Features

- Emit LoadLyricCache signal on lyric cache load

### ⚙️ Miscellaneous Tasks

- Release v0.3.7
## [0.3.6] - 2024-04-29

### 🚀 Features

- *(tray)* Refetch-lyric for windows
- Disable loading local lyric
- Allow LRC line that starts with [xxx] without ':'
- Select local LRC file to import

### 🐛 Bug Fixes

- Missing import
- Disable local lrc hint on `enable-local-lyric=false`
- *(typo)* Import-lrc -> import-lyric

### 💼 Other

- Migrate to ksni 0.2.2

### 🚜 Refactor

- Store dbus connection instead of gapplication
- Change net-test to offline-test
- Inline variable "text"

### 📚 Documentation

- *(install)* Add flatpak installation link
- *(install-en)* Add flatpak installation link

### ⚙️ Miscellaneous Tasks

- Add archlinuxcn in installation guide
- Release v0.3.6
## [0.3.5] - 2024-04-25

### 🚀 Features

- Reload lyric from cache
- Emit NewLyricCache signal on lyric cache update

### ⚙️ Miscellaneous Tasks

- Release v0.3.5
## [0.3.4] - 2024-04-25

### ⚙️ Miscellaneous Tasks

- Release v0.3.4
## [0.3.3] - 2024-04-25

### ⚙️ Miscellaneous Tasks

- *(metainfo)* H3 tags are not supported
- *(desktopfile)* Remove action `Launch`
- Release v0.3.3
- *(metainfo)* Fix warnings
- *(metainfo)* Add screenshot of search window
- *(metainfo)* Use v0.3.3 tag
## [0.3.2] - 2024-04-25

### 🐛 Bug Fixes

- *(readme)* Broken link of desktop file
- Enum tuple variant cannot be destrcucted by tuple matching

### 💼 Other

- *(tray)* Do not flood start_time on smtc

### ⚙️ Miscellaneous Tasks

- Make extracting values from tuples more readable
- Use for loop to refactor entries appending in imp.vbox
- Use for loop to refactor items appending in ui_section & play_section
- Release v0.3.2
## [0.3.1] - 2024-04-24

### 🚀 Features

- Set search-window ColumnView row padding in themes

### 📚 Documentation

- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Release v0.3.1
- Fix test.yml
- Remove microsoft-prod.list
- Fix documents deploy CI
## [0.3.0] - 2024-04-24

### 🐛 Bug Fixes

- *(i18n)* En_US translation
- Set titlebar before set decorated
- *(ci)* New schema path

### 💼 Other

- Mpris position delay
- Set time diff log level to TRACE so we will not flood debug log

### 🚜 Refactor

- Pass WeakRef<Window> to register_sync_task

### 📚 Documentation

- *(install)* Update packaging script
- *(changelog)* Update changelog

### ◀️ Revert

- "Translate display modes"
## [0.2.21] - 2024-04-23

### 🚀 Features

- Fix secondary lyric will not end
- Blacklist players by name/identity
- Initial support for Windows SMTC
- Full mouse click-through for windows
- Initial tray-icon support for windows
- Restart in tray-icon for windows
- Hide lyric on pause
- Convert zh-hans/zh-hant in fuzzy match with opencc
- Use song_search_detailed in search box to apply aliases

### 🐛 Bug Fixes

- Wrong import path
- Duplicated import
- Do workarounds for windows-rs bug
- Misuse of `windows-rs`
- Windows smtc position
- Feature gate for unix should be `cfg(unix)`
- Gtk4 freezes on `Window::close` on windows
- Set paused as false after resumed to playing
- Use '/' as splitte on searchbox creating
- Underscore not showing in display mode menu
- *(i18n)* Load i18n on windows
- Stop lyric update when show_lyric_on_pause not set on pause

### 💼 Other

- *(rpm)* Packit is no longer needed [ci skip]
- *(deps)* Bump actions/checkout from 3 to 4
- *(deps)* Bump orhun/git-cliff-action from 2 to 3
- *(deps)* Bump actions/upload-artifact from 3 to 4
- Make gettext, openssl, journald optional
- Add icon for win32 build
- Support LastUpdateTime on windows
- Print control status at trace level
- *(windows)* Add build script
- Fine-tune fuzzy match factor
- Make fuzzy-match weight more than length based match
- Apply alias for artist name on `netease`
- Enable i18n for msvc build

### 🚜 Refactor

- Use `bind_shortcut` in switch-passthrough
- Extract `try_sync_track` and `reconnect_player` from `interop::*`
- Rename `list_player_names` to `list_players`
- Define OS-specified helpers in a trait
- Cleanup old code

### 📚 Documentation

- *(changelog)* Update changelog
- *(readme)* Musixfox-go released the required patch [ci skip]
- *(readme)* AutoLyric `C/C++`
- *(install)* Build on windows
- *(readme)* Intro windows user directories
- *(changelog)* Update changelog
- *(readme)* The only compatible player on windows
- *(changelog)* Update changelog
- *(readme)* Update missing translation
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(build)* Gettext-rs on windows cannot builds out-of-box with MSVC
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(changelog)* Update changelog

### ⚙️ Miscellaneous Tasks

- Example environment is no longer required [ci skip]
- Automatically update AUR PKGBUILD
- Reset pkgrel after a pkgver bump
- *(test)* Cleanup unused imports
- Switch to upstream `tray-icon`
- Bump dependencies

### ◀️ Revert

- "Update README.md"
## [Setup] - 2024-03-01

### 🚀 Features

- Set Priority::HIGH for lyric_scroll
- Select labels by origin/translation in theme
- Add theme no-background [ci skip]

### 💼 Other

- Replace extension with PathBuf::set_extension

### 🚜 Refactor

- Remove unneeded player_meta helper
- Implement Debug for dyn LyricProvider
- Remove repeated bind-shortcut code
- Do not set display mode manually
- Use builder api in widget setup

### 📚 Documentation

- *(install)* Show packaging status
- *(build)* Update packaging docs
- *(build)* Fix install command for schema [ci skip]
- *(readme)* Go-musicfox merged fix-position [ci skip]
- *(changelog)* Update changelog
- *(readme)* List recommended players in chart [ci skip]
- *(readme)* Complain some bad support [ci skip]
- *(readme)* Intro listen1 [ci skip]

### ⚙️ Miscellaneous Tasks

- Remove CSS stylelintrc
- Fix missing bracket
## [0.2.11] - 2024-02-23

### 🚀 Features

- Invoke app actions with mpsc::channel
- Invoke ui actions with mpsc::channel
- Set unsupported reason on below label
- Hide default text on idle
- Generate comments for config
- Set lyric display mode in run time
- Initial tray-icon support
- *(tray-icon)* PlayAction control
- Optionally start tray-icon service
- Restart waylyrics in tray-icon
- Fuzzy-match lyrics with Sorensen-Dice coefficient
- Skip fuzzy-match if we got only title in metadata
- Sort search result by fuzzy-match weight
- Show tooltip for search entries
- I18n support
- Only show set-lyric button when avaliable
- Show icons on tray-icon menu
- [**breaking**] Set lyric align mode on run time
- Set lyric-display-mode in GSettings
- Split popover menu to UI section and Play section
- Set mouse click-through in GSettings
- *(theme)* New style
- (optional) select labels by translation/origin

### 🐛 Bug Fixes

- Should re-export PlayAction
- Re-export play-action channel
- Do not hide below label in set_lyric
- Remove `hide_label_on_empty_text = false` support
- Do not show lyric on `Stopped` status
- Do not hide below label in set_lyric
- Remove `hide_label_on_empty_text = false` support
- Append comments with `documented`
- Repeated docs appends to commented config.toml
- Missing feature gate for re-export in sync
- Changelog CI need write permission
- Incomplete i18n in menu
- Restart in tray-icon set click-through
- Use hsla for transparent window background
- Hsl color format
- Show_both mode place origin to above if no translation

### 💼 Other

- *(logo)* Make logo not so shining in dark background [ci skip]
- *(logo)* White shade for text and logo
- Make tray-icon feature optional
- Log reveiced action event
- Rename feature to 'action-event' rather than tray-icon
- Impl real tray-icon functions
- Remove unnessacary global variable for PlayerId's
- Log detailed likelihood with trace-level
- Migrate to better dice-coefficient lib
- *(rpm)* Add icon
- Make global theme presets optional
- Add translation for Simplefied Chinese
- *(rpm)* Package with i18n files
- Translate length to 时长
- Translate display_mode
- Translate lyric align
- Switch-passthrough in tray-icon without restart

### 🚜 Refactor

- Rename AppAction to PlayAction
- Move UI-related actions to crate::app::actions
- Rename to play_action
- Move search_window to crate::app
- Move search_window to crate::app
- Move config.rs to config/
- Remove repeated function prefix
- Skip fuzzy-match in it's block
- Cleanup `set_lyric_with_mode` [ci skip]
- Move logic-related fields to Window::new
- Use `clone!` macro rather than calling upgrade/downgrade manually

### 📚 Documentation

- *(contribute)* Introduce Conventional Commit [ci skip]
- *(readme)* Add logo and intro video [ci skip]
- *(readme)* Move chat banners to center block [ci skip]
- *(readme)* Waybar supports all wlroots-based compositors
- *(config)* Lyric-align accepts CamelCase value
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(readme)* Update readme
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(build)* Add gettext to dependencies
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(changelog)* Update changelog
- *(readme)* Remove dead `flutter-netease-music`
- *(changelog)* Update changelog

### ⚡ Performance

- Store default MainContext

### 🎨 Styling

- Remove unused Downgrade import
- Package_name as a constant [ci-skip]
- *(theme)* Format CSS with Prettier and Stylelint

### ⚙️ Miscellaneous Tasks

- *(dep)* Migrate to gtk4-rs 0.8.0
- Generate changelogs for master
- Update changelog on each commit
- Do not flood commit logs
- Remove unnessacary use of Arc<dyn LyricProvider>
- Fix changelog ci skipped unexpectedly
- Remove invalid github template
- Clean up theme comments
- Disable changelog on tag due to shitty failure [ci skip]

### ◀️ Revert

- "feat: (optional) select labels by translation/origin"
## [0.2.3] - 2024-02-13

### 🚀 Features

- Lyric play
- Custom font family
- Sync with mpris2
- Allow auto hide empty label & code refactor
- #8
- Rpm spec (#31)
- Disconnect from current player if received SIGUSR1, fix #43
- Switch decoration on SIGUSR2, fix #49
- Add new fields to old configuration
- Place menubutton at end
- Switch passthrough
- Change project icon
- Switch decoration with shortcut
- Reload theme
- Reload lyric (from cache)
- Switch click-through with Alt-P
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
- *(icon)* New design
- Try load lyric from disk
- Metadata from LyricHint (for music_file without local lyrics)
- *(build)* Improve RPM group
- Add support for musicfox
- Intro `is_likely_songid` for songid verification
- Set empty label explictly
- Allow to show origin lyric in above
- Add `trans` theme [ci skip]

### 🐛 Bug Fixes

- Correct variable name typo in lib.rs
- Better Gtk CSD UI
- Clickthrough not working with decoration
- Cannot restore decoration after set invisable
- *(ci)* Gen_config_example was removed
- Translation lyric not showing
- *(ci)* OpenSUSE packaging
- *(ci)* Update smoketest.yml
- Lyrics are not trimed
- *(ci)* Do not try build doc on PR
- Get songmid for lyric query
- XML decode for LRC from QQMusic api
- Set serde default for Triggers
- Return Ok when fetch successfully
- Refetch lyric should ignore cache
- *(ci)* RPM distros packaging
- *(icon)* Add white background
- Set default on empty lyric status
- Deepsource: anti-pattern use of
- Recorrect player_name part for NCM-gtk4
- Missing white background
- *(icon)* Missing white color
- LyricHint::File from mpris should decode with `url::Url::to_file_path`
- Unconfigured provider from hint will cause lyrics not to be loaded
- Set default lyric_update interval to 50ms
- Skip UTF-8 BOM so lrc-nom will work
- Cannot return value referencing function parameter
- Desktop file does not need a launch action [ci skip]
- Ignore invalid LRC lines
- Override user theme
- Hint support for musicfox
- QQMusic::init should panic on `Err()` rather than `Ok()`
- Set 20ms as default lyric update interval

### 💼 Other

- Query play status and sync on update
- Fix not fetching lyrics
- Remote/master
- Label alignment
- Adjust layout about switch player
- Switch-click-through-shortcut
- Direct id support for YesPlayMusic
- Do not copy template themes
- Ship with a generated template is useless
- Save GTK+ CSD state
- Impl search & fetch_lyric
- Disable QQMusic source by default
- Trim lyrics on set
- *(ui)* Set Align::Start for column entries
- *(ui)* Better layout for search_window
- Log to journalctld
- Set resizeable for title, singer and album
- Asynchronously fetch lyric
- *(ui)* Distingush label for whether lyrics were cached
- Apply Mutex<()> lock for update_lyrics
- Album+title for qqmusic
- *(lyric/qqmusic)* Handle -1901 error
- Fix mpris player connect
- Use an external build system
- Info - player hint
- Log errors on local lyric loading
- Let install to make directory
- *(rpm)* Package LICENSE and README
- *(rpm)* Fix install
- *(ui)* Drop `SIGUSR` control support
- Add ability to disable tests require network

### 🚜 Refactor

- Sync::player
- Interop/lyric-fetch/lyric-scroll
- Replace unneeded field pattern with `..`
- Setup helpers
- Move out update_lyric
- Make `init_dirs` a public method so we could write tests
- Remove `CONFIG_HOME`
- Impl into_owned for LyricLine
- Setup `QQMusic` with `init()` call
- Rename `get_lyric` to `parse_lyric` [ci skip]
- Rename confusing `match_lyric` to `verify_lyric` [ci skip]
- Apply clippy fix [ci skip]

### 📚 Documentation

- Fix typo and add schema compilation
- Explain triggers fields
- Mention default shortcuts
- Update outdated requirement
- Note to install pango
- *(readme)* Remove `lx-music` because it's dead [ci skip]
- Define `lrc_iter` behaviour
- *(install)* Explain build environment variable
- *(install)* Download pre-built executables [ci skip]
- Play with musicfox need position fix patch
- Add `osdlyrics` to alternatives [ci skip]
- Explain more fields in `Config`
- Firefox via Plasma Integration [ci skip]
- `lollypop`, GTK3-based local music player
- Restate outdated doc of `TrackMeta`

### 🎨 Styling

- *(build)* Use RPM macros to replace hard-coded directories

### 🧪 Testing

- Add test for `get_lyric_path`
- Fix get_lrc_path doctesr
- Test netease lyric get & parse
- Test LRC parsing
- Move unit tests to inside `src/`
- Move out doctest for `get_lrc_path`
- Add unit tests for QQMusic::init

### ⚙️ Miscellaneous Tasks

- Migrate to anyhow::Result
- *(ci)* Add smoketest
- *(ci)* Rust caching and weston
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
- *(ci)* Build with nightly toolchain
- Remove some unreachable code
- Remove some unreachable code
- Use Cell for T: Copy
- Typing for TrackState
- `extract connect_factory`
- Add debug msg for config and theme path
- *(log)* Also log result weight
- Replace `RefCell` with `Cell`
- Add debug msg for get_cache_path
- Add dhat.out.* to .gitignore
- Add mimalloc to dependencies
- Avoid hard coding a must be same value twice
- *(search_window)* Log selected song id
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
- *(doc)* Mention desktop file for global shortcut
- Replace `with_borrow_mut` with `set` [ci skip]
- Clean up lyric scroll
- *(doc)* Update alternatives [ci skip]
- *(doc)* Update translation [ci skip]
- Drop support for ncm-gtk legacy name
- *(doc)* Update doc for youtube-music [ci skip]
- *(doc)* Add `musicfox`, a TUI based music player [ci skip]
- Remove unused import
- Run real test in CI
- Test qqmusic provider initializing
- Remove unreachable `title.unwrap_or()` call
- Migrate to dtolnay/rust-toolchain
