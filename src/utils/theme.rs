use futures::StreamExt;

use crate::{glib_spawn, log, THEME_PATH};

pub fn auto_theme_change(
    color_scheme: impl AsRef<str>,
    theme_dark_switch: bool,
) -> anyhow::Result<()> {
    let color_scheme = color_scheme.as_ref();
    let Some(settings) = gtk::Settings::default() else {
        return Ok(());
    };

    match color_scheme {
        "light" => settings.set_gtk_application_prefer_dark_theme(false),
        "dark" => settings.set_gtk_application_prefer_dark_theme(true),
        "auto" => {
            if dark_light::detect() == dark_light::Mode::Dark {
                settings.set_gtk_application_prefer_dark_theme(true);
                if theme_dark_switch {
                    set_and_update(true);
                }
            } else {
                settings.set_gtk_application_prefer_dark_theme(false)
            }

            // Listen to changes...
            glib_spawn!(async move {
                let mut stream = match dark_light::subscribe().await {
                    Ok(stream) => stream,
                    Err(e) => {
                        log::error!("Subscribing color-scheme changing events failed: {e}");
                        return;
                    }
                };
                while let Some(mode) = stream.next().await {
                    match mode {
                        dark_light::Mode::Dark => {
                            settings.set_gtk_application_prefer_dark_theme(true);
                            if theme_dark_switch {
                                set_and_update(true);
                            }
                        }
                        _ => {
                            settings.set_gtk_application_prefer_dark_theme(false);
                            if theme_dark_switch {
                                set_and_update(false);
                            }
                        }
                    }
                }
            });
        }
        _ => {
            anyhow::bail!("Unknown color-scheme {}", color_scheme);
        }
    }

    Ok(())
}

// Check system color scheme
fn replace_suffix<'a>(input: &'a str, old_suffix: &str, new_suffix: &str) -> String {
    if input.ends_with(old_suffix) {
        let trimmed = &input[..input.len() - old_suffix.len()];
        format!("{}{}", trimmed, new_suffix)
    } else {
        input.to_string()
    }
}

fn set_and_update(dark: bool) {
    THEME_PATH.with_borrow_mut(|theme_path| {
        let filename: &str = match theme_path.file_name().and_then(|p| p.to_str()) {
            Some(p) => p,
            None => return,
        };
        if dark {
            if !filename.ends_with("-dark.css") {
                let new_name = replace_suffix(filename, ".css", "-dark.css");
                theme_path.set_file_name(new_name);
            }
        } else {
            if filename.ends_with("-dark.css") {
                let new_name = replace_suffix(filename, "-dark.css", ".css");
                theme_path.set_file_name(new_name);
            }
        }

        if let Ok(style) = std::fs::read_to_string(&theme_path) {
            crate::app::utils::merge_css(&style);
        } else {
            log::warn!("Filename {:?} not found.", theme_path);
        }
    })
}
