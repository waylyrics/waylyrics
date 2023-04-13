use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    window: Window,
    text: Text,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Text {
    top: Offset,
    /// RGBA
    color: (u8, u8, u8, u8),
    /// RGBA
    background: (u8, u8, u8, u8),
}

#[derive(Deserialize, Serialize, Default)]
pub struct Window {
    left: Offset,
    top: Offset,
}

#[derive(Deserialize, Serialize)]
pub struct Offset {
    offset_type: OffsetType,
    value: f64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OffsetType {
    Scale,
    Pixel,
}

impl Default for Offset {
    fn default() -> Self {
        Self { offset_type: OffsetType::Scale, value: 0. }
    }
}
