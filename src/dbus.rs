use zbus::{proxy};
use zvariant::{OwnedValue};
use std::collections::HashMap;

#[proxy(
    interface = "org.freedesktop.DBus.Properties",
    default_service = "org.mpris.MediaPlayer2.yandex-music",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Properties {
    fn get(
        &self,
        interface_name: &str,
        property_name: &str
    ) -> zbus::Result<OwnedValue>;

    #[zbus(signal)]
    fn properties_changed(
        &self,
        interface_name: &str,
        changed_properties: HashMap<String, OwnedValue>,
        invalidated_properties: Vec<String>,
    );
}
