use std::collections::HashMap;
use zvariant::{OwnedValue};

#[derive(Debug)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub art_url: String,
    pub(crate) length_us: i64,
}

pub fn parse_metadata(meta: &HashMap<String, OwnedValue>) -> Option<Track> {
    let title = meta
        .get("xesam:title")?
        .downcast_ref::<String>()
        .unwrap_or("Unknown Title".to_string());

    let artist = meta
        .get("xesam:artist")
        .and_then(|v| {
            let vec: Vec<OwnedValue> = v.clone().try_into().ok()?;
            let names: Vec<String> = vec.into_iter()
                .filter_map(|val| val.try_into().ok())
                .collect();
            if names.is_empty() { None } else { Some(names.join(", ")) }
        })
        .unwrap_or("Unknown Artist".to_string());

    let album = meta
        .get("xesam:album")?
        .downcast_ref::<String>()
        .ok();

    let art_url = meta
        .get("mpris:artUrl")?
        .downcast_ref::<String>()
        .unwrap_or("Unknown".to_string());


    let length_us = meta
        .get("mpris:length")?
        .downcast_ref::<i64>()
        .unwrap_or(-1);

    Some(Track {
        title,
        artist,
        album,
        art_url,
        length_us,
    })
}
