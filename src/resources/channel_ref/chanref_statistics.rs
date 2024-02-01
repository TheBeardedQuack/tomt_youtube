use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            Channel as RscType,
            ChannelStats,
        },
        channel_ref::ChannelRef,
        Resource, RscPart,
    },
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct StatsRef<'yt>(ChannelRef<'yt>);

impl<'yt> StatsRef<'yt>
{
    pub async fn hide_subs_count(
        &self
    ) -> Result<bool, YtError> {
        let stats = self.fetch().await?;
        Ok(stats.hide_subs_count)
    }

    pub async fn subs_count(
        &self
    ) -> Result<u64, YtError> {
        let stats = self.fetch().await?;
        Ok(stats.subs_count)
    }

    pub async fn video_count(
        &self
    ) -> Result<u64, YtError> {
        let stats = self.fetch().await?;
        Ok(stats.video_count)
    }

    pub async fn view_count(
        &self
    ) -> Result<u64, YtError> {
        let stats = self.fetch().await?;
        Ok(stats.view_count)
    }
    
    pub async fn fetch(
        &self
    ) -> Result<ChannelStats, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.stats {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl RscPart<RscType>
for StatsRef<'_> {
    type Backing = ChannelStats;

    const PART_NAME: &'static str = "statistic";
    const PART_KEY: PartKey = PartKey::Statistics;
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
