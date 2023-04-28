use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub text_color: Rgba,
    pub background_color: Rgba,
    pub font_size: u16,
    pub font_family: Option<String>,
}

type Rgba = (u8, u8, u8, u8);

impl Default for Config {
    fn default() -> Self {
        Self {
            text_color: (255, 255, 255, 255),
            background_color: (0, 0, 0, 0),
            font_size: 40,
            font_family: None,
        }
    }
}
