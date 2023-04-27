use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub text: Text,
}

#[derive(Deserialize, Serialize)]
pub struct Text {
    /// RGBA
    pub color: (u8, u8, u8, u8),
}

impl Default for Text {
    fn default() -> Self {
        Self {
            color: (255, 255, 255, 255),
        }
    }
}
