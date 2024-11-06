use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QQMusicConfig {
    /// QQMusicApi api url
    /// example: "http://127.0.0.1:11451"
    pub api_base_url: Option<String>,
}
