use gtk::prelude::*;
use gtk::{ButtonsType, Window};

#[allow(deprecated, reason = "GTK >= 4.10 is not yet a hard requirement")]
pub fn show_dialog(parent: Option<&impl IsA<Window>>, msg: &str, level: gtk::MessageType) {
    let msg_dialog =
        gtk::MessageDialog::new(parent, gtk::DialogFlags::MODAL, level, ButtonsType::Ok, msg);
    msg_dialog.connect_response(|dialog, _| {
        dialog.close();
    });
    msg_dialog.present();
}
