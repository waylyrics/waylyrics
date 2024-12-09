use documented::DocumentedFields;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, DocumentedFields)]
#[serde(default)]
pub struct QQMusicConfig {
    /// QQMusicApi api url
    ///
    /// example: "http://127.0.0.1:11451"
    pub api_base_url: String,

    /// QQMusic Cookies
    ///
    /// WARN: this will apply to the whole QQMusicApi instance
    pub cookies: Option<String>,
}

impl Default for QQMusicConfig {
    fn default() -> Self {
        Self {
            api_base_url: "http://127.0.0.1:3300".into(),
            cookies: Default::default(),
        }
    }
}
