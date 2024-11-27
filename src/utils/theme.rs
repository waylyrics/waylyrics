use futures::StreamExt;

use crate::{config::ColorScheme, glib_spawn, log, THEME_PATH};

pub fn auto_theme_change(color_scheme: ColorScheme, theme_dark_switch: bool) {
    let Some(settings) = gtk::Settings::default() else {
        return;
    };

    match color_scheme {
        ColorScheme::Light => settings.set_gtk_application_prefer_dark_theme(false),
        ColorScheme::Dark => settings.set_gtk_application_prefer_dark_theme(true),
        ColorScheme::Auto => {
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
    }
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
