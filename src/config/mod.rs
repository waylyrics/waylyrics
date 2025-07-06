use documented::DocumentedFields;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};

use crate::lyric_providers::{netease::Netease, qqmusic::QQMusicConfig, LyricProvider};

#[derive(Clone, Copy, Default, EnumIter, strum::Display, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
    #[default]
    Auto,
    Dark,
    Light,
}

#[derive(Clone, Copy, Default, EnumIter, strum::Display, EnumString)]
pub enum Align {
    /// left align
    Start,
    /// right align
    End,
    #[default]
    Center,
    Fill,
}

#[derive(Clone, Copy, Default, EnumIter, strum::Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum LyricDisplayMode {
    #[default]
    ShowBoth,
    ShowBothRev,
    Origin,
    PreferTranslation,
}

#[derive(Deserialize, Serialize, DocumentedFields)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    /// auto connect to a player if last was disconnected/waylyrics was idle
    pub auto_connect: bool,
    /// the interval waylyrics updates position/metadata from player
    pub player_sync_interval: String,

    /// the interval waylyrics refreshes lyric labels
    pub lyric_update_interval: String,

    /// waylyrics matches lyrics with `weights`
    ///
    /// if `(length-lyric_len).abs() < length_toleration`,
    ///
    /// waylyrics set it's weight as zero, mark it a best choice
    pub length_toleration: String,

    /// whether to cache lyrics
    ///
    /// note: persistenced lyric offset depends on this
    pub cache_lyrics: bool,

    /// theme to load (<name>.css)
    pub theme: String,

    /// if enabled, lyrics match one or more `filter_regex` will be hidden
    pub enable_filter_regex: bool,

    /// if enabled, when waylyrics find `xesam:url`
    ///
    /// starts with `file://` and no any other hint was matched,
    ///
    /// waylyrics will try to read the `.lrc` file has same filename
    pub enable_local_lyric: bool,

    /// if enabled, when waylyrics loads a local lyric file,
    ///
    /// it will try to extract translated lyric
    pub extract_translated_lyric: bool,

    /// hide lyric if it matches any of these regexies
    ///
    /// inspired by LyricX's filter list
    pub filter_regexies: Vec<String>,

    /// avaliable options: 网易云音乐, QQ音乐, LRCLib
    pub lyric_search_source: Vec<String>,

    /// if enabled, waylyrics will set `DEFAULT_TEXT` on idle,
    ///
    /// otherwise it just show nothing
    pub show_default_text_on_idle: bool,

    /// if enabled, waylyrics will show lyric on `Pause`
    pub show_lyric_on_pause: bool,

    /// whether to run tray-icon service
    #[cfg(feature = "tray-icon")]
    pub show_tray_icon: bool,

    /// player with these name will be ignored
    pub player_name_blacklist: Vec<String>,

    /// player with these identity will be ignored
    pub player_identity_blacklist: Vec<String>,

    /// shortcuts when focusing on waylyrics
    ///
    /// for global ones, please install the `.desktop` file
    ///
    /// also check trigger format at https://docs.gtk.org/gtk4/ctor.ShortcutTrigger.parse_string.html
    pub triggers: Triggers,

    /// QQMusic config
    ///
    /// With `cookies` as `name=value; name1=value1;` format,
    /// waylyrics will set cookies for the QQMusicApi service at startup.
    pub qqmusic: QQMusicConfig,

    /// Color scheme used for Gtk interface -- light, dark or auto (use system)
    ///
    /// Not supported on Windows.
    pub color_scheme: ColorScheme,

    /// Whether to use <name>-dark.css when system is in dark mode
    pub theme_dark_switch: bool,
}

/// check [GTK+'s official document](https://docs.gtk.org/gtk4/ctor.ShortcutTrigger.parse_string.html) for trigger format
#[derive(Deserialize, Serialize, DocumentedFields)]
#[serde(default)]
pub struct Triggers {
    /// whether to show GTK+ CSD
    pub switch_decoration: String,
    /// reapply current theme file
    pub reload_theme: String,
    /// manually search lyric
    pub search_lyric: String,
    /// try to refetch lyric
    pub refetch_lyric: String,
    /// whether to allow mouse click-through
    pub switch_passthrough: String,
}

impl Default for Triggers {
    fn default() -> Self {
        Self {
            switch_decoration: "<Control>d".into(),
            reload_theme: "<Control><Shift>t".into(),
            search_lyric: "<Control>s".into(),
            refetch_lyric: "<Alt><Shift>l".into(),
            switch_passthrough: "<Alt>p".into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_connect: true,
            player_sync_interval: "2s".to_owned(),
            lyric_update_interval: "20ms".to_owned(),
            length_toleration: "2s".to_owned(),
            theme: "default".into(),
            cache_lyrics: true,
            enable_filter_regex: false,
            enable_local_lyric: true,
            filter_regexies: default_filter_regexies(),
            lyric_search_source: vec![Netease.unique_name().into()],
            show_default_text_on_idle: true,
            show_lyric_on_pause: true,
            #[cfg(feature = "tray-icon")]
            show_tray_icon: true,
            player_name_blacklist: vec!["firefox".into()],
            player_identity_blacklist: vec![],
            extract_translated_lyric: true,
            triggers: Triggers::default(),
            qqmusic: QQMusicConfig::default(),
            color_scheme: ColorScheme::default(),
            theme_dark_switch: false,
        }
    }
}

impl From<Align> for gtk::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Start => Self::Start,
            Align::End => Self::End,
            Align::Center => Self::Center,
            Align::Fill => Self::Fill,
        }
    }
}

#[rustfmt::skip]
fn default_filter_regexies() -> Vec<String> {
    [
        "^作词", "^作詞", "^作曲", "^編曲", "^编曲", "^収録", "^收录", "^演唱", "^歌手", "^歌曲", "^制作", "^製作", "^歌词",
        "^歌詞", "^翻譯", "^翻译", "^插曲", "^插入歌", "^主题歌", "^主題歌", "^片頭曲", "^片头曲", "^片尾曲", "^SoundTrack",
        "^アニメ",
    ]
    .map(str::to_string)
    .to_vec()
}

mod merge;
pub use merge::append_comments;
