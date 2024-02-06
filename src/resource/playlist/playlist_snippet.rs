use serde::Deserialize;

use crate::resource::{
    channel::ChannelId,
    playlist::{
        Playlist as RscType,
        PlaylistPart,
    },
    thumbnail::ThumbnailList,
    Resource, RscPart,
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct PlaylistSnippet
{
    #[serde(rename = "channelId")]
    pub channel_id: ChannelId,

    pub title: String,

    pub description: String,

    pub thumbnails: ThumbnailList,

    #[serde(rename = "channelTitle")]
    pub channel_title: String,
}

impl RscPart<RscType>
for PlaylistSnippet
{
    type Backing = Self;

    const PART_KEY: PartKey = PlaylistPart::Snippet;
    const PART_NAME: &'static str = "snippet";
}
