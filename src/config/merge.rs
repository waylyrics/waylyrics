use std::fmt::Write;

use anyhow::Result;
use documented::DocumentedFields;
use toml_edit::DocumentMut;

use crate::config::Config;

pub fn append_comments(toml: &str) -> Result<String> {
    let mut new_doc = toml.parse::<DocumentMut>()?;

    for (mut field, item) in new_doc.iter_mut() {
        if let Ok(doc) = Config::get_field_docs(field.replace('-', "_")) {
            let doc = doc
                .lines()
                .filter(|s| !s.is_empty())
                .fold(String::new(), |mut s, l| {
                    let _ = write!(&mut s, "\n# {l}");
                    s
                });
            match item {
                toml_edit::Item::Value(_) => field.leaf_decor_mut().set_prefix(doc + "\n"),
                toml_edit::Item::Table(t) => t.decor_mut().set_suffix(doc),
                _ => (),
            }
        }
    }

    Ok(new_doc.to_string())
}
