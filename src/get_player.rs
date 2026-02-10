use sysinfo::System;
use zbus::Connection;
use zbus::names::BusName;

fn is_yandex_music_running() -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();

    sys.processes().values().any(|proc| {
        let name: String = proc.name().to_string_lossy().into_owned();
        name.contains("yandexmusic")
    })
}

pub async fn find_yandex_player(conn: &Connection) -> anyhow::Result<BusName<'static>> {
    let proxy = zbus::fdo::DBusProxy::new(conn).await?;
    let names = proxy.list_names().await?;

    for name in names {
        if name.starts_with("org.mpris.MediaPlayer2") && is_yandex_music_running() {
            return Ok(name.into());
        }
    }

    anyhow::bail!("Yandex Music player not found")
}