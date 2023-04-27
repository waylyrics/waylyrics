use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub text_color: Rgba,
    pub background_color: Rgba,
}

type Rgba = (u8, u8, u8, u8);

impl Default for Config {
    fn default() -> Self {
        Self {
            text_color: (255, 255, 255, 255),
            background_color: (0, 0, 0, 0),
        }
    }
}
