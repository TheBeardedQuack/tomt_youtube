use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            ChannelData as RscType,
            ChannelDetails, DetailsData,
            indirect::ChannelRef,
        }, Resource, RscId, RscPart
    },
};

type PartKey = <RscType as Resource>::PartKey;

use std::{
    future::Future,
    pin::pin,
    task::{ready, Poll},
};

#[derive(Clone, Debug)]
pub struct DetailsRef<'yt>(ChannelRef<'yt>);

impl ChannelDetails
for DetailsRef<'_>
{
    async fn special_playlists(
        &self
    ) -> Result<crate::resources::channel::SpecialPlaylists, YtError> {
        todo!()
    }
}

impl RscPart<RscType>
for DetailsRef<'_> {
    type Backing = DetailsData;

    const PART_NAME: &'static str = "contentDetails";
    const PART_KEY: PartKey = PartKey::Details;
}

impl<'yt> DetailsRef<'yt>
{
    async fn run(
        &self
    ) -> Result<DetailsData, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.details {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl Future
for DetailsRef<'_>
{
    type Output = Result<DetailsData, YtError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> Poll<Self::Output> {
        Poll::Ready(ready!(pin!(self.run()).poll(cx)))
    }
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
