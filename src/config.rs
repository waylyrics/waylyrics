use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    window: Window,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Window {
    position: Position,
    text_position: Position,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Position {
    top_off: f64,
    left_off: f64,
}
