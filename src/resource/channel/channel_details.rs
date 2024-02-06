use crate::resource::{
    channel::Channel as RscType,
    playlist::PlaylistId,
    Resource, RscPart,
};

use serde::Deserialize;

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct ChannelDetails
{
    #[serde(rename = "relatedPlaylists")]
    pub playlists: SpecialPlaylists,
}

impl RscPart<RscType>
for ChannelDetails {
    type Backing = ChannelDetails;

    const PART_NAME: &'static str = "contentDetails";
    const PART_KEY: PartKey = PartKey::Details;
}

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct SpecialPlaylists
{
    pub likes: PlaylistId,
    pub uploads: PlaylistId
}
