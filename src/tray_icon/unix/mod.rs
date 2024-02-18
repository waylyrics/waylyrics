use ksni::{Tray, TrayService};

struct TrayIcon;

impl Tray for TrayIcon {
    fn icon_name(&self) -> String {
        crate::APP_ID.to_string()
    }
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
}

pub fn start_tray_service() {
    let service = TrayService::new(TrayIcon);
    service.spawn();
}
