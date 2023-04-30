use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub text_color: Rgba,
    pub background_color: Rgba,
    pub font_size: u16,
    pub font_family: Option<String>,
    pub mpris_sync_interval: Duration,
    pub lyric_update_interval: Duration,
}

type Rgba = (u8, u8, u8, u8);

impl Default for Config {
    fn default() -> Self {
        Self {
            text_color: (255, 255, 255, 255),
            background_color: (0, 0, 0, 0),
            font_size: 40,
            font_family: None,
            mpris_sync_interval: Duration::from_secs(3),
            lyric_update_interval: Duration::from_millis(80),
        }
    }
}
