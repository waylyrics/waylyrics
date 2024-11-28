use anyhow::Result;

#[cfg(unix)]
pub fn acquire_instance_name() -> Result<()> {
    #[cfg(feature = "multi-monitor")]
    {
        use std::hash::Hasher;
        use std::time::Duration;

        use dbus::blocking::Connection;
        use gtk::glib::ffi::g_random_int_range;
        use hex_simd::AsOut;

        let conn = Connection::new_session()?;
        let proxy = conn.with_proxy(
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            Duration::from_secs(5),
        );
        let (names,): (Vec<String>,) =
            proxy.method_call("org.freedesktop.DBus", "ListNames", ())?;
        let waylyrics_instances = names
            .iter()
            .filter(|name| name.starts_with(crate::APP_ID_FIXED))
            .map(String::as_str)
            .collect::<ahash::AHashSet<_, _>>();
        if !waylyrics_instances.contains(crate::APP_ID_FIXED) {
            let _ = crate::INSTANCE_NAME.set(String::from(crate::APP_ID_FIXED));
        } else {
            let random_num = unsafe { g_random_int_range(i32::MIN, i32::MAX) };
            let mut hasher = ahash::AHasher::default();
            hasher.write_i32(random_num);
            let hash = hasher.finish();
            let mut out: Vec<u8> = vec![0; 16];
            let hash_str = hex_simd::encode_as_str(
                &hash.to_ne_bytes(),
                out.as_out(),
                hex_simd::AsciiCase::Lower,
            );
            let instance_name = format!("{}-{}", crate::APP_ID_FIXED, hash_str);
            let _ = crate::INSTANCE_NAME.set(instance_name);
        }
    }
    #[cfg(not(feature = "multi-monitor"))]
    {
        let _ = crate::INSTANCE_NAME.set(String::from(crate::APP_ID_FIXED));
    }

    Ok(())
}

#[cfg(not(unix))]
pub fn acquire_instance_name() -> Result<()> {
    let _ = waylyrics::INSTANCE_NAME.set(String::from(waylyrics::APP_ID_FIXED));
}
