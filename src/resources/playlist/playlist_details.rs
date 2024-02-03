use crate::resources::{
    playlist::{
        Playlist as RscType,
        PlaylistParts,
    },
    Resource, RscPart,
};

use serde::Deserialize;

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct PlaylistDetails
{
    #[serde(rename = "itemCount")]
    pub video_count: u32,
}

impl RscPart<RscType>
for PlaylistDetails
{
    type Backing = Self;

    const PART_KEY: PartKey = PlaylistParts::Details;
    const PART_NAME: &'static str = "contentDetails";
}
