use serde::{Deserialize, Serialize};

use crate::lyric_providers::{netease::Netease, LyricProvider};

#[derive(Deserialize, Serialize, Clone, Copy)]
#[serde(tag = "type")]
pub enum Align {
    /// left align
    Start,
    /// right align
    End,
    /// (default)
    Center,
    Fill,
}

#[derive(Deserialize, Serialize, Clone, Copy, Default)]
#[serde(rename_all = "snake_case")]
pub enum LyricDisplay {
    #[default]
    ShowBoth,
    Origin,
    PreferTranslation,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    pub player_sync_interval: String,
    pub lyric_align: Align,
    pub lyric_update_interval: String,
    pub length_toleration: String,
    /// note: persistenced lyric offset depends on this
    pub cache_lyrics: bool,
    pub click_pass_through: bool,
    pub hide_label_on_empty_text: bool,
    pub theme: String,
    pub enable_filter_regex: bool,
    /// inspired by LyricX's filter [list](https://github.com/ddddxxx/LyricsX/blob/c16b6a413dda7bc0b793b897522e0c4ee0ffc716/LyricsX/Supporting%20Files/UserDefaults.plist#L31-L62)
    pub filter_regexies: Vec<String>,
    pub triggers: Triggers,
    pub qqmusic_api_base_url: Option<String>,
    /// avaliable options: 网易云音乐, QQ音乐
    pub lyric_search_source: Vec<String>,
    pub lyric_display_mode: LyricDisplay,
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
            lyric_update_interval: "50ms".to_owned(),
            length_toleration: "2s".to_owned(),
            click_pass_through: true,
            hide_label_on_empty_text: true,
            theme: "default".into(),
            cache_lyrics: true,
            enable_filter_regex: false,
            filter_regexies: vec![
                "^作词".into(),
                "^作詞".into(),
                "^作曲".into(),
                "^編曲".into(),
                "^编曲".into(),
                "^収録".into(),
                "^收录".into(),
                "^演唱".into(),
                "^歌手".into(),
                "^歌曲".into(),
                "^制作".into(),
                "^製作".into(),
                "^歌词".into(),
                "^歌詞".into(),
                "^翻譯".into(),
                "^翻译".into(),
                "^插曲".into(),
                "^插入歌".into(),
                "^主题歌".into(),
                "^主題歌".into(),
                "^片頭曲".into(),
                "^片头曲".into(),
                "^片尾曲".into(),
                "^SoundTrack".into(),
                "^アニメ".into(),
            ],
            lyric_align: Align::Center,
            triggers: Triggers::default(),
            qqmusic_api_base_url: None,
            lyric_search_source: vec![Netease.unique_name().into()],
            lyric_display_mode: LyricDisplay::default(),
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
