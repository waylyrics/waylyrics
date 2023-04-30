use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// should be valid Css Color
    pub background_color: String,
    pub mpris_sync_interval: String,
    pub lyric_update_interval: String,

    pub origin: Font,
    pub translated: Option<Font>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Font {
    /// should be valid Css Color
    pub text_color: String,
    pub font_size: u16,
    pub font_family: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background_color: "rgba (0, 0, 0, 0)".to_owned(),
            mpris_sync_interval: "3s".to_owned(),
            lyric_update_interval: "80ms".to_owned(),
            origin: Font {
                text_color: "rgba (255, 255, 255, 255)".to_owned(),
                font_size: 40,
                font_family: None,
            },
            translated: None,
        }
    }
}
