use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QQMusicConfig {
    /// QQMusicApi api url
    ///
    /// example: "http://127.0.0.1:11451"
    pub api_base_url: Option<String>,

    /// QQMusic Cookies
    ///
    /// WARN: this will apply to the whole QQMusicApi instance
    pub cookies: Option<String>,
}
