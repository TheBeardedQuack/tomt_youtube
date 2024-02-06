use crate::resource::{
    playlist::{
        Playlist as RscType,
        PlaylistPart,
    },
    privacy::Privacy,
    Resource, RscPart,
};

use serde::Deserialize;

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct PlaylistStatus
{
    #[serde(rename = "privacyStatus")]
    pub privacy: Privacy
}

impl RscPart<RscType>
for PlaylistStatus
{
    type Backing = Self;

    const PART_KEY: PartKey = PlaylistPart::Status;
    const PART_NAME: &'static str = "status";
}
