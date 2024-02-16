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
    RefetchLyric,
    RemoveLyric,
    SearchLyric,
}

fn register_play_action(app: WeakRef<Application>) -> Sender<PlayAction> {
    let (tx, rx) = async_channel::unbounded();

    glib::spawn_future_local(async move {
        while let Ok(action) = rx.recv().await {
            debug!("Received Play Action {action:?}");
            let (action_name, parameter) = match action {
                PlayAction::Connect(player_id) => ("connect", Some(player_id.to_variant())),
                PlayAction::Disconnect => ("disconnect", None),
                PlayAction::RefetchLyric => ("refetch-lyric", None),
                PlayAction::RemoveLyric => ("remove-lyric", None),
                PlayAction::SearchLyric => ("search-lyric", None),
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
