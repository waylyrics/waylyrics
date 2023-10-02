use std::cell::{Cell, RefCell};

use glib::Properties;
use glib_macros;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[derive(Default)]
pub struct ResultData {
    pub id: String,
    pub title: String,
    pub singer: String,
    pub album: String,
    pub length: u64,
    // glib seems do not support dyn so I use u8 index instead
    pub provider_idx: u8,
}

#[derive(Properties, Default)]
#[properties(wrapper_type = super::ResultObject)]
pub struct ResultObject {
    #[property(name = "title", get, set, type = String, member = title)]
    #[property(name = "singer", get, set, type = String, member = singer)]
    #[property(name = "album", get, set, type = String, member = album)]
    #[property(name = "length", get, set, type = u64, member = length)]
    #[property(name = "id", get, set, type = String, member = id)]
    #[property(name = "provider-idx", get, set, type = u8, member = provider_idx)]
    pub data: RefCell<ResultData>,
}

#[glib::object_subclass]
impl ObjectSubclass for ResultObject {
    const NAME: &'static str = "SearchResultObject";
    type Type = super::ResultObject;
}

#[glib_macros::derived_properties]
impl ObjectImpl for ResultObject {}

#[derive(Default)]
pub struct Window {
    pub vbox: gtk::Box,

    pub input: gtk::Entry,

    pub set_button: gtk::Button,

    pub result_scrolled_window: gtk::ScrolledWindow,
    pub result_list: gtk::ColumnView,
    pub result_title: gtk::ColumnViewColumn,
    pub result_singer: gtk::ColumnViewColumn,
    pub result_album: gtk::ColumnViewColumn,
    pub result_length: gtk::ColumnViewColumn,
    pub results: RefCell<Option<gio::ListStore>>,

    pub use_cache: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "SearchWindow";
    type Type = super::Window;
    type ParentType = gtk::Window;
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_results();

        obj.setup_ui();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
