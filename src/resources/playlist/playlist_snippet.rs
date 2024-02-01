use serde::Deserialize;

use crate::resources::{channel::ChannelId, thumbnail::ThumbnailList};

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
