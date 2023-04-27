use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub text: Text,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Text {
    pub top: Offset,
    /// RGBA
    pub color: (u8, u8, u8, u8),
    /// RGBA
    pub background: (u8, u8, u8, u8),
}

#[derive(Deserialize, Serialize)]
pub struct Offset {
    pub scale: f64,
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            scale: 0.1,
        }
    }
}
