use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            Channel as RscType,
            ChannelDetails,
            SpecialPlaylists,
        },
        Resource, RscPart
    },
    request::channel::ChannelRequest,
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct DetailsRequest<'yt>(ChannelRequest<'yt>);

impl<'yt> DetailsRequest<'yt>
{
    pub async fn special_playlists(
        &self
    ) -> Result<SpecialPlaylists, YtError> {
        let deets = self.fetch().await?;
        Ok(deets.playlists)
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
for DetailsRequest<'_>
{
    type Backing = ChannelDetails;

    const PART_NAME: &'static str = "contentDetails";
    const PART_KEY: PartKey = PartKey::Details;
}

impl<'yt> From<ChannelRequest<'yt>>
for DetailsRequest<'yt>
{
    fn from(
        value: ChannelRequest<'yt>
    ) -> Self {
        Self(value)
    }
}
