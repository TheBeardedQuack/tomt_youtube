use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            Channel as RscType,
            ChannelStatus,
        },
        privacy::Privacy,
        Resource, RscPart,
    },
    request::channel::ChannelRequest,
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct StatusRequest<'yt>(ChannelRequest<'yt>);

impl<'yt> StatusRequest<'yt>
{
    pub async fn privacy(
        &self
    ) -> Result<Privacy, YtError> {
        let status = self.fetch().await?;
        Ok(status.privacy)
    }

    pub async fn fetch(
        &self
    ) -> Result<ChannelStatus, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match chan_data.status {
            Some(status) => Ok(status),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl RscPart<RscType>
for StatusRequest<'_>
{
    type Backing = ChannelStatus;

    const PART_NAME: &'static str = "status";
    const PART_KEY: PartKey = PartKey::Status;
}

impl<'yt> From<ChannelRequest<'yt>>
for StatusRequest<'yt>
{
    fn from(
        value: ChannelRequest<'yt>
    ) -> Self {
        Self(value)
    }
}
