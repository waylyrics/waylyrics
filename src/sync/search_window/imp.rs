use std::cell::RefCell;

use glib::Properties;
use glib_macros;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[derive(Default)]
pub struct ResultData {
    pub name: String,
}

#[derive(Properties, Default)]
#[properties(wrapper_type = super::ResultObject)]
pub struct ResultObject {
    #[property(name = "name", get, set, type = String, member = name)]
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

    pub result_list: gtk::ListView,
    pub results: RefCell<Option<gio::ListStore>>,
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
