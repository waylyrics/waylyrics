use ahash::HashMap;
use once_cell::sync::Lazy;
const ARTIST_ALIASES: [(&str, &str); 1] = [("jay chou", "周杰伦")];
pub static ARTIST_ALIAS_MAP: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| ARTIST_ALIASES.into_iter().collect());
