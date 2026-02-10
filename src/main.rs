use std::collections::HashMap;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use zbus::Connection;
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::fdo::PropertiesProxy;
use zvariant::{ObjectPath, OwnedValue};

mod get_player;
mod dbus;
mod metadata;
mod discord_rpc;

use crate::get_player::find_yandex_player;
use crate::discord_rpc::{discord_timestamps, update_rpc};
use crate::metadata::parse_metadata;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = Connection::session().await?;
    let player = find_yandex_player(&conn).await?;

    let mut rpc = DiscordIpcClient::new("1466858776178196742"); // You APP ID
    rpc.connect()?;

    let props = PropertiesProxy::builder(&conn)
        .destination(player)?
        .path(ObjectPath::try_from("/org/mpris/MediaPlayer2")?)?
        .build()
        .await?;

    let mut stream = props.receive_properties_changed().await?;

    while let Some(signal) = stream.next().await {
        let args = signal.args()?;
        if let Some(value) = args.changed_properties.get("Metadata") {
            let meta = value.clone().downcast::<HashMap<String, OwnedValue>>()?;
            println!("=============");
            println!("{:?}", meta);
            println!("=============");
            println!("{:?}", parse_metadata(&meta));
            if let Some(track) = parse_metadata(&meta) {

                let pos = props
                    .get("org.mpris.MediaPlayer2.Player".try_into()?, "Position")
                    .await
                    .ok()
                    .and_then(|v| v.downcast_ref::<i64>().ok())
                    .clone()
                    .unwrap_or(0);

                let (start, end) = discord_timestamps(pos, track.length_us);
                update_rpc(&mut rpc, &track, start, end)?;
            }
        }
    }

    Ok(())
}
