use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            Channel as RscType,
            ChannelDetails,
            SpecialPlaylists,
        },
        channel_ref::ChannelRef,
        Resource, RscPart
    },
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct DetailsRef<'yt>(ChannelRef<'yt>);

impl<'yt> DetailsRef<'yt>
{
    pub async fn special_playlists(
        &self
    ) -> Result<SpecialPlaylists, YtError> {
        let deets = self.fetch().await?;
        Ok(deets.playlists)
    }

    pub async fn run(
        &self
    ) -> Result<ChannelDetails, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.details {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }

    pub async fn fetch(
        &self
    ) -> Result<ChannelDetails, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.details {
            Some(deets) => Ok(deets.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl RscPart<RscType>
for DetailsRef<'_> {
    type Backing = ChannelDetails;

    const PART_NAME: &'static str = "contentDetails";
    const PART_KEY: PartKey = PartKey::Details;
}

impl<'yt> From<ChannelRef<'yt>>
for DetailsRef<'yt>
{
    fn from(
        value: ChannelRef<'yt>
    ) -> Self {
        Self(value)
    }
}
