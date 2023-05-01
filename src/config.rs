use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// should be valid Css Color
    pub background_color: String,
    pub mpris_sync_interval: String,
    pub lyric_update_interval: String,

    pub origin: LabelSettings,
    pub translated: Option<LabelSettings>,
    pub allow_click_through_me: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LabelSettings {
    pub font: Font,
    /// should be valid Css Color
    pub bg_color: String,
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
            background_color: "rgba(0, 0, 0, 0.0)".to_owned(),
            mpris_sync_interval: "3s".to_owned(),
            lyric_update_interval: "80ms".to_owned(),
            origin: LabelSettings {
                font: Font {
                    text_color: "rgba(255, 255, 255, 1.0)".to_owned(),
                    font_size: 40,
                    font_family: None,
                },
                bg_color: "rgba(0, 0, 0, 0.3)".to_owned(),
            },
            translated: None,
            allow_click_through_me: true,
        }
    }
}
