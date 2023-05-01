use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mpris_sync_interval: String,
    pub lyric_update_interval: String,
    pub allow_click_through_me: bool,
    pub full_width_lyric_bg: bool,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            mpris_sync_interval: "3s".to_owned(),
            lyric_update_interval: "20ms".to_owned(),
            allow_click_through_me: true,
            full_width_lyric_bg: false,
        }
    }
}
