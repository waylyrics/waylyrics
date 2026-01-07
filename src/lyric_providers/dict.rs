use std::sync::LazyLock;

use ahash::HashMap;

const ARTIST_ALIASES: [(&str, &str); 1] = [("jay chou", "周杰伦")];
pub static ARTIST_ALIAS_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| ARTIST_ALIASES.into_iter().collect());
