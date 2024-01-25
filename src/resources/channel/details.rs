use crate::{
    error::YtError,
    resources::{
        channel::ChannelData as RscType,
        playlist::PlaylistId,
        Resource, RscPart,
    }
};

use serde::{Deserialize, Serialize};

type PartKey = <RscType as Resource>::PartKey;

pub trait ChannelDetails
{
    fn special_playlists(
        &self
    ) -> impl std::future::Future<Output = Result<SpecialPlaylists, YtError>> + Send;
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct DetailsData
{
    #[serde(rename = "relatedPlaylists")]
    pub playlists: SpecialPlaylists,
}

impl RscPart<RscType>
for DetailsData {
    type Backing = DetailsData;

    const PART_NAME: &'static str = "contentDetails";
    const PART_KEY: PartKey = PartKey::Details;
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct SpecialPlaylists
{
    pub likes: Vec<PlaylistId>,
    pub uploads: Vec<PlaylistId>
}

impl ChannelDetails
for DetailsData
{
    async fn special_playlists(
        &self
    ) -> Result<SpecialPlaylists, YtError> {
        Ok(self.playlists.clone())
    }
}
