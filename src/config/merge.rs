use std::fmt::Write;

use anyhow::Result;
use documented::DocumentedFields;
use toml_edit::{DocumentMut, Item, Table};

use crate::config::{Config, QQMusicConfig, Triggers};

pub fn append_comments(toml: &str) -> Result<String> {
    let mut new_doc = toml.parse::<DocumentMut>()?;

    comment_sub_struct(new_doc.as_table_mut(), Config::get_field_docs);
    Ok(new_doc.to_string())
}

fn comment_sub_struct(
    t: &mut Table,
    get_field: fn(String) -> Result<&'static str, documented::Error>,
) {
    for (mut field, item) in t.iter_mut() {
        let Ok(doc) = get_field(field.replace('-', "_")).map(fold_doc) else {
            continue;
        };

        match item {
            Item::Value(_) => field.leaf_decor_mut().set_prefix(doc + "\n"),
            Item::Table(t) => {
                t.decor_mut().set_suffix(doc);

                match field.get() {
                    "triggers" => {
                        comment_sub_struct(t, Triggers::get_field_docs);
                    }
                    "qqmusic" => {
                        comment_sub_struct(t, QQMusicConfig::get_field_docs);
                    }
                    _ => (),
                }
            }
            _ => {}
        }
    }
}

fn fold_doc(doc: &str) -> String {
    doc.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .fold(String::default(), |mut s, l| {
            let _ = write!(&mut s, "\n# {l}");
            s
        })
}
