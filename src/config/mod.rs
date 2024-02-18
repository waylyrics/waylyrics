use documented::DocumentedFields;
use serde::{Deserialize, Serialize};

use crate::lyric_providers::{netease::Netease, LyricProvider};

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub struct AlignS {
    #[serde(rename = "type")]
    align_type: Align,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
pub enum Align {
    /// left align
    Start,
    /// right align
    End,
    #[default]
    Center,
    Fill,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum LyricDisplay {
    #[default]
    ShowBoth,
    ShowBothRev,
    Origin,
    PreferTranslation,
}

#[derive(Deserialize, Serialize, DocumentedFields)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    /// the interval waylyrics updates position/metadata from player
    pub player_sync_interval: String,

    /// the interval waylyrics refreshes lyric labels
    pub lyric_update_interval: String,

    /// waylyrics matches lyrics with `weights`
    /// if `(length-lyric_len).abs() < length_toleration`,
    /// waylyrics set it's weight as zero, mark it a best choice
    pub length_toleration: String,

    /// whether to cache lyrics
    /// note: persistenced lyric offset depends on this
    pub cache_lyrics: bool,

    /// whether to allow mouse-click passthrough
    pub click_pass_through: bool,

    /// theme to load (<name>.css)
    pub theme: String,

    /// if enabled, lyrics match one or more `filter_regex` will be hidden
    pub enable_filter_regex: bool,

    /// hide lyric if it matches any of these regexies
    /// inspired by LyricX's filter list
    pub filter_regexies: Vec<String>,

    /// QQMusicApi api url
    /// example: "http://127.0.0.1:11451"
    pub qqmusic_api_base_url: Option<String>,

    /// avaliable options: 网易云音乐, QQ音乐
    pub lyric_search_source: Vec<String>,

    /// lyric display mode
    /// avaliable options:
    /// - `show_both`: show origin and translated lyric
    /// - `show_both_rev`: similiar to `show_both`, but origin text are showed below
    /// - `prefer_translation`: show translated lyric if found any, or show origin lyric
    /// - `origin`: only to show origin lyric
    pub lyric_display_mode: LyricDisplay,

    /// if enabled, waylyrics will set `DEFAULT_TEXT` on idle,
    /// otherwise it just show nothing
    pub show_default_text_on_idle: bool,

    /// the way two lyric label align in
    /// possible values: Center, Start, End, Fill
    /// also check [GTK+ doc](https://docs.gtk.org/gtk4/enum.Align.html#members)
    pub lyric_align: AlignS,

    /// shortcuts when focusing on waylyrics
    /// for global ones, please install the `.desktop` file
    /// also check trigger format at https://docs.gtk.org/gtk4/ctor.ShortcutTrigger.parse_string.html
    pub triggers: Triggers,
}

/// check [GTK+'s official document](https://docs.gtk.org/gtk4/ctor.ShortcutTrigger.parse_string.html) for trigger format
#[derive(Deserialize, Serialize)]
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
            player_sync_interval: "2s".to_owned(),
            lyric_update_interval: "20ms".to_owned(),
            length_toleration: "2s".to_owned(),
            click_pass_through: true,
            theme: "default".into(),
            cache_lyrics: true,
            enable_filter_regex: false,
            filter_regexies: default_filter_regexies(),
            lyric_align: AlignS::default(),
            triggers: Triggers::default(),
            qqmusic_api_base_url: None,
            lyric_search_source: vec![Netease.unique_name().into()],
            lyric_display_mode: LyricDisplay::default(),
            show_default_text_on_idle: true,
        }
    }
}

impl From<AlignS> for gtk::Align {
    fn from(value: AlignS) -> Self {
        match value.align_type {
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
