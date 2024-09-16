use async_channel::Sender;
use std::sync::OnceLock;

use gtk::{
    glib::{self, WeakRef},
    prelude::*,
    Application,
};

use crate::log::debug;

#[derive(Clone, Debug)]
pub enum PlayAction {
    Connect(String),
    Disconnect,
    ReloadLyric,
    RefetchLyric,
    RemoveLyric,
    SearchLyric,
    #[cfg(feature = "import-lyric")]
    ImportOriginalLyric,
    #[cfg(feature = "import-lyric")]
    ImportTranslatedLyric,
    #[cfg(feature = "export-lyric")]
    ExportOriginalLyric,
    #[cfg(feature = "export-lyric")]
    ExportTranslatedLyric,
}

fn register_play_action(app: WeakRef<Application>) -> Sender<PlayAction> {
    let (tx, rx) = async_channel::unbounded();

    glib::spawn_future_local(async move {
        while let Ok(action) = rx.recv().await {
            debug!("Received Play Action {action:?}");
            let (action_name, parameter) = match action {
                PlayAction::Connect(player_id) => ("connect", Some(player_id.to_variant())),
                PlayAction::Disconnect => ("disconnect", None),
                PlayAction::ReloadLyric => ("reload-lyric", None),
                PlayAction::RefetchLyric => ("refetch-lyric", None),
                PlayAction::RemoveLyric => ("remove-lyric", None),
                PlayAction::SearchLyric => ("search-lyric", None),
                #[cfg(feature = "import-lyric")]
                PlayAction::ImportOriginalLyric => ("import-lyric", Some(true.to_variant())),
                #[cfg(feature = "import-lyric")]
                PlayAction::ImportTranslatedLyric => ("import-lyric", Some(false.to_variant())),
                #[cfg(feature = "export-lyric")]
                PlayAction::ExportOriginalLyric => ("export-lyric", Some(true.to_variant())),
                #[cfg(feature = "export-lyric")]
                PlayAction::ExportTranslatedLyric => ("export-lyric", Some(false.to_variant())),
            };

            if let Some(app) = app.upgrade() {
                app.activate_action(action_name, parameter.as_ref());
            }
        }
    });

    tx
}

pub fn init_play_action_channel(app: WeakRef<Application>) {
    let tx = register_play_action(app);
    PLAY_ACTION.set(tx).expect("must only initialize once");
}

pub static PLAY_ACTION: OnceLock<Sender<PlayAction>> = OnceLock::new();
