use chrono::Utc;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

use crate::metadata::Track;

pub fn discord_timestamps(position_us: i64, length_us: i64) -> (i64, i64) {
    let now = Utc::now().timestamp();
    let start = now - (position_us / 1_000_000);
    let end = start + (length_us / 1_000_000);
    (start, end)
}

pub fn update_rpc(
    rpc: &mut DiscordIpcClient,
    track: &Track,
    start: i64,
    end: i64,
) -> anyhow::Result<()> {
    let mut act = activity::Activity::new()
        .details(format!("{} — {}", track.artist, track.title))
        .assets(
            activity::Assets::new()
                .large_image(&track.art_url)
                .large_text("Yandex Music"),
        )
        .activity_type(activity::ActivityType::Listening);

    if let Some(album) = &track.album {
        act = act.state(album.clone());
    }

    rpc.set_activity(act)?;
    Ok(())
}
