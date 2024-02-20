mod imp;

use std::sync::Arc;

use crate::log::{error, info};
use crate::lyric_providers::SongInfo;
use gettextrs::gettext;
use glib::Object;
use gtk::glib::{clone, IntoGStr};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ColumnViewColumn};
use gtk::{prelude::*, ListItem};
use tokio::task::JoinSet;

use crate::{glib_spawn, tokio_spawn, LYRIC_PROVIDERS};

use crate::app::dialog::show_dialog;
use crate::sync::lyric::cache::update_lyric_cache;
use crate::sync::{fuzzy_match_song, get_lyric_cache_path, set_current_lyric, LyricState};

glib::wrapper! {
    pub struct ResultObject(ObjectSubclass<imp::ResultObject>);
}

impl ResultObject {
    pub fn new(
        id: String,
        title: String,
        singer: String,
        album: String,
        length: u64,
        provider_idx: usize,
        provider_name: &'static str,
    ) -> Self {
        Object::builder()
            .property("title", title)
            .property("singer", singer)
            .property("album", album)
            .property("length", length)
            .property("id", id)
            .property("provider-idx", provider_idx as u8)
            .property("provider-name", provider_name)
            .build()
    }
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new<S: IntoGStr>(title: S, album: S, artists: S, use_cache: bool) -> Self {
        let window: Self = Object::builder().build();
        window.set_title(Some(&gettext("Search lyric")));

        window.imp().input_title.buffer().set_text(title);
        window.imp().input_album.buffer().set_text(album);
        window.imp().input_artists.buffer().set_text(artists);

        window.imp().use_cache.set(use_cache);
        window
    }

    fn results(&self) -> gio::ListStore {
        self.imp()
            .results
            .borrow()
            .clone()
            .expect("Could not get results")
    }

    fn setup_results(&self) {
        let model = gio::ListStore::new::<ResultObject>();

        self.imp().results.replace(Some(model));

        let selection_model = gtk::SingleSelection::new(Some(self.results()));
        self.imp().result_list.set_model(Some(&selection_model));
    }

    fn setup_ui(&self) {
        let imp = self.imp();

        imp.vbox
            .set_properties(&[("orientation", &gtk::Orientation::Vertical)]);

        imp.vbox.append(&imp.input_title);
        imp.vbox.append(&imp.input_artists);
        imp.vbox.append(&imp.input_album);

        imp.vbox.append(&imp.result_scrolled_window);
        imp.vbox.append(&imp.set_button);

        imp.result_scrolled_window.set_child(Some(&imp.result_list));
        imp.result_scrolled_window.set_vexpand(true);
        imp.result_scrolled_window
            .set_hscrollbar_policy(gtk::PolicyType::Never);
        imp.result_scrolled_window.set_height_request(300);
        imp.result_scrolled_window.set_width_request(300);

        imp.column_title.set_expand(true);
        imp.column_singer.set_resizable(true);
        imp.column_album.set_resizable(true);

        imp.column_title.set_title(Some(&gettext("Title")));
        imp.column_singer.set_title(Some(&gettext("Singer")));
        imp.column_album.set_title(Some(&gettext("Album")));
        imp.column_length.set_title(Some(&gettext("Length")));
        imp.column_source.set_title(Some(&gettext("Source")));

        imp.result_list.append_column(&imp.column_title);
        imp.result_list.append_column(&imp.column_singer);
        imp.result_list.append_column(&imp.column_album);
        imp.result_list.append_column(&imp.column_length);
        imp.result_list.append_column(&imp.column_source);

        imp.input_title.set_placeholder_text(Some(&gettext("Enter title...")));
        imp.input_title.set_tooltip_text(Some(&gettext("title")));
        imp.input_album.set_placeholder_text(Some(&gettext("Enter album...")));
        imp.input_album.set_tooltip_text(Some(&gettext("album")));
        imp.input_artists
            .set_placeholder_text(Some(&gettext("Enter artists...")));
        imp.input_artists.set_tooltip_text(Some(&gettext("artists")));

        imp.input_title
            .set_secondary_icon_name(Some("system-search-symbolic"));

        imp.set_button.set_label(&gettext("Set as lyric"));

        self.set_child(Some(&imp.vbox));
    }

    async fn search(&self) {
        let buffer = self.imp().input_title.buffer();
        let query_title = Arc::new(buffer.text().to_string());
        let buffer = self.imp().input_album.buffer();
        let query_album = Arc::new(buffer.text().to_string());
        let buffer = self.imp().input_artists.buffer();
        let query_artists = Arc::new(buffer.text().to_string());
        if [&query_album, &query_artists, &query_title]
            .iter()
            .all(|q| q.is_empty())
        {
            return;
        }

        let mut results = vec![];
        let providers = LYRIC_PROVIDERS
            .get()
            .expect("lyric providers should be initialized");

        let _title = query_title.clone();
        let _album = query_album.clone();
        let _artists = query_artists.clone();
        let mut results = tokio_spawn!(async move {
            let mut set = JoinSet::new();
            let query = format!("{_title} {_album} {_artists}");
            for (idx, provider) in providers.iter().enumerate() {
                let query = query.clone();
                let provider_id = provider.unique_name();
                set.spawn(
                    async move { (provider.search_song(&query).await, provider_id, idx, query) },
                );
            }

            while let Some(Ok((search_result, provider_name, idx, query))) = set.join_next().await {
                let tracks = match search_result {
                    Ok(songs) => songs,
                    Err(e) => {
                        // TODO: to show errors to users in GUI
                        error!("{e} occurs when search {query} on {}", provider_name);
                        continue;
                    }
                };
                for track in tracks {
                    // ResultObject contains RefCell, so it cannot be used inside tokio_spawn!()
                    results.push((track, idx, provider_name));
                }
            }
            results
        })
        .await
        .expect("Tokio runtime failure");

        let query_title_chars = query_title.chars().collect::<Vec<_>>();
        let query_album_chars = query_album.chars().collect::<Vec<_>>();
        let query_artists_chars = query_artists.chars().collect::<Vec<_>>();
        results.sort_unstable_by_key(
            |(
                SongInfo {
                    title,
                    album,
                    singer,
                    ..
                },
                ..,
            )| {
                let factor = fuzzy_match_song(
                    &query_title_chars,
                    Some(&query_album_chars),
                    Some(&query_artists_chars),
                    &title.chars().collect::<Vec<_>>(),
                    album
                        .as_ref()
                        .map(|a| a.chars().collect::<Vec<_>>())
                        .as_deref(),
                    &singer.chars().collect::<Vec<_>>(),
                );

                (factor * -1024.) as i32
            },
        );

        let results: Vec<ResultObject> = results
            .into_iter()
            .map(|(track, idx, provider_name)| {
                ResultObject::new(
                    track.id,
                    track.title,
                    track.singer,
                    track.album.unwrap_or_default(),
                    track.length.as_secs(),
                    idx,
                    provider_name,
                )
            })
            .collect();
        self.results().remove_all();

        if results.is_empty() {
            show_dialog(Some(self),&gettext( "No result was found."), gtk::MessageType::Error);
            return;
        }

        self.results().extend_from_slice(&results);
    }

    fn get_selected_result(&self) -> Option<ResultObject> {
        let selection_model = self
            .imp()
            .result_list
            .model()
            .and_downcast::<gtk::SingleSelection>()
            .expect("Needs to be SingleSelection");
        let selected = selection_model.selected();
        if selected == gtk::INVALID_LIST_POSITION {
            return None;
        }
        let result = self
            .results()
            .item(selected)
            .and_downcast::<ResultObject>()
            .expect("Needs to be ResultObject");
        Some(result)
    }

    fn setup_callbacks(&self) {
        let imp = self.imp();
        imp.input_title
            .connect_activate(clone!(@weak self as window => move |_| {
                let window = window.downgrade();
                crate::log::debug!("spawned window.search() from search button");
                glib_spawn!(async move {
                    if let Some(window) = window.upgrade() {
                        window.search().await
                    }
                });
            }));

        imp.input_title
            .connect_icon_release(clone!(@weak self as window => move |_, _| {
                let window = window.downgrade();
                crate::log::debug!("spawned window.search() from search icon release");
                glib_spawn!(async move {
                    if let Some(window) = window.upgrade() {
                        window.search().await
                    }
                });
            }));

        imp.set_button
            .connect_clicked(clone!(@weak self as window => move |_| {
                let result = window.get_selected_result();
                let Some(result) = result else {
                    return;
                };

                let provider_idx = result.provider_idx() as usize;
                let Some(provider) = LYRIC_PROVIDERS
                    .get()
                    .expect("lyric providers must be initialized")
                    .get(provider_idx) else {
                    error!("provider_idx {} is out of range", provider_idx);
                    return;
                };

                info!("selected {} from {}", result.id(), provider.unique_name());

                crate::log::debug!("spawned query_lyric from set lyric button");
                glib_spawn!(async move {
                    let song_id = result.id();
                    match provider.query_lyric(&song_id).await {
                        Ok(lyric) => {
                            let origin = provider.parse_lyric(&lyric);
                            let translation = provider.parse_translated_lyric(&lyric);
                            set_current_lyric(LyricState{ origin, translation });


                            if window.imp().use_cache.get() {
                                if let Some(cache_path) = get_lyric_cache_path() {
                                    update_lyric_cache(&cache_path);
                                }
                            }
                        },
                        Err(e) => {
                            let error_msg = format!("{e} when getting lyric.");
                            error!(error_msg);
                            show_dialog(Some(&window), &error_msg, gtk::MessageType::Error);
                        }
                    }
                });
            }));
    }

    fn setup_factory(&self) {
        let imp = self.imp();
        connect_factory(&imp.column_title, |result| result.title(), true);
        connect_factory(&imp.column_singer, |result| result.singer(), false);
        connect_factory(&imp.column_album, |result| result.album(), true);
        connect_factory(
            &imp.column_length,
            |result| format_length(result.length()),
            false,
        );
        connect_factory(&imp.column_source, |result| result.provider_name(), false);
    }
}

fn connect_factory(
    column: &ColumnViewColumn,
    get_field: impl 'static + Fn(ResultObject) -> String,
    wrap: bool,
) {
    let factory = gtk::SignalListItemFactory::new();
    factory.connect_setup(move |_, list_item| {
        let label = gtk::Label::new(None);
        label.set_halign(gtk::Align::Start);
        label.set_wrap(wrap);
        label.set_wrap_mode(gtk::pango::WrapMode::Word);
        list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .set_child(Some(&label));
    });
    factory.connect_bind(move |_, list_item| {
        let result_object = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .item()
            .and_downcast::<ResultObject>()
            .expect("Needs to be ResultObject");
        let label = list_item
            .downcast_ref::<ListItem>()
            .expect("Needs to be ListItem")
            .child()
            .and_downcast::<gtk::Label>()
            .expect("Needs to be Label");

        label.set_label(&get_field(result_object));
    });

    column.set_factory(Some(&factory));
}

fn format_length(length: u64) -> String {
    let min = length / 60;
    let sec = length % 60;
    format!("{min:02}:{sec:02}")
}
