use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            Channel as RscType,
            ChannelStatus,
        },
        channel_ref::ChannelRef,
        privacy::Privacy,
        Resource, RscPart,
    },
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct StatusRef<'yt>(ChannelRef<'yt>);

impl<'yt> StatusRef<'yt>
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
for StatusRef<'_>
{
    type Backing = ChannelStatus;

    const PART_NAME: &'static str = "status";
    const PART_KEY: PartKey = PartKey::Status;
}

impl<'yt> From<ChannelRef<'yt>>
for StatusRef<'yt>
{
    fn from(
        value: ChannelRef<'yt>
    ) -> Self {
        Self(value)
    }
}
