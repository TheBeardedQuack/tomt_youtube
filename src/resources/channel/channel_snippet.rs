use crate::resources::{
    channel::Channel as RscType,
    thumbnail::ThumbnailList,
    Resource, RscPart
};

use serde::Deserialize;

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Deserialize)]
pub struct ChannelSnippet
{
    pub title: String,
    pub description: String,
    pub custom_url: Option<String>,
    pub thumbnails: ThumbnailList,
}

impl RscPart<RscType>
for ChannelSnippet {
    type Backing = ChannelSnippet;

    const PART_KEY: PartKey = PartKey::Snippet;
    const PART_NAME: &'static str = "snippet";
}
