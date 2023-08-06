use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", default)]
pub struct Config {
    pub mpris_sync_interval: String,
    pub lyric_align: Align,
    pub lyric_update_interval: String,
    pub length_toleration: String,
    pub cache_lyrics: bool,
    pub window_decoration: bool,
    pub click_pass_through: bool,
    pub hide_label_on_empty_text: bool,
    pub origin_lyric_in_above: bool,
    pub theme: String,
    pub enable_filter_regex: bool,
    pub filter_regexies: Vec<String>,
    /// check [GTK+'s official document](https://docs.gtk.org/gtk4/ctor.ShortcutTrigger.parse_string.html) for trigger format
    pub switch_decoration_trigger: String,
    /// check [GTK+'s official document](https://docs.gtk.org/gtk4/ctor.ShortcutTrigger.parse_string.html) for trigger format
    pub switch_passthrough_trigger: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mpris_sync_interval: "2s".to_owned(),
            lyric_update_interval: "20ms".to_owned(),
            length_toleration: "2s".to_owned(),
            click_pass_through: true,
            hide_label_on_empty_text: true,
            theme: "default".into(),
            origin_lyric_in_above: true,
            cache_lyrics: true,
            enable_filter_regex: false,
            window_decoration: true,
            /// inspired by LyricX's filter [list](https://github.com/ddddxxx/LyricsX/blob/c16b6a413dda7bc0b793b897522e0c4ee0ffc716/LyricsX/Supporting%20Files/UserDefaults.plist#L31-L62)
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
                "^アニメ)".into(),
            ],
            lyric_align: Align::Center,
            switch_decoration_trigger: "<Control>d".into(),
            switch_passthrough_trigger: "<Alt>p".into(),
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
