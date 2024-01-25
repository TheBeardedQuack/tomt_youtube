use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            ChannelData as RscType,
            ChannelStats, StatsData,
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
pub struct StatsRef<'yt>(ChannelRef<'yt>);

impl ChannelStats
for StatsRef<'_>
{
    async fn hide_subs_count(
        &self
    ) -> Result<bool, YtError> {
        self.clone().await?
            .hide_subs_count().await
    }

    async fn subs_count(
        &self
    ) -> Result<u64, YtError> {
        self.clone().await?
            .subs_count().await
    }

    async fn video_count(
        &self
    ) -> Result<u64, YtError> {
        self.clone().await?
            .video_count().await
    }

    async fn view_count(
        &self
    ) -> Result<u64, YtError> {
        self.clone().await?
            .view_count().await
    }
}

impl RscPart<RscType>
for StatsRef<'_> {
    type Backing = StatsData;

    const PART_NAME: &'static str = "statistic";
    const PART_KEY: PartKey = PartKey::Statistics;
}

impl<'yt> StatsRef<'yt>
{
    async fn run(
        &self
    ) -> Result<StatsData, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.stats {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl Future
for StatsRef<'_>
{
    type Output = Result<StatsData, YtError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> Poll<Self::Output> {
        Poll::Ready(ready!(pin!(self.run()).poll(cx)))
    }
}

impl<'yt> From<ChannelRef<'yt>>
for StatsRef<'yt>
{
    fn from(
        value: ChannelRef<'yt>
    ) -> Self {
        Self(value)
    }
}
