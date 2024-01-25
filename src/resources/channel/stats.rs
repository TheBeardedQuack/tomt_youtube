use serde::{Deserialize, Serialize};

use crate::{error::YtError, resources::{
    channel::ChannelData as RscType,
    Resource, RscPart
}};

type PartKey = <RscType as Resource>::PartKey;

pub trait ChannelStats
{
    fn hide_subs_count(
        &self
    ) -> impl std::future::Future<Output = Result<bool, YtError>> + Send;

    fn subs_count(
        &self
    ) -> impl std::future::Future<Output = Result<u64, YtError>> + Send;

    fn video_count(
        &self
    ) -> impl std::future::Future<Output = Result<u64, YtError>> + Send;
    
    fn view_count(
        &self
    ) -> impl std::future::Future<Output = Result<u64, YtError>> + Send;
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct StatsData
{
    #[serde(rename = "hiddenSubscriberCount")]
    hide_subs_count: bool,

    #[serde(rename = "subscriberCount")]
    subs_count: u64,

    #[serde(rename = "videoCount")]
    video_count: u64,

    #[serde(rename = "viewCount")]
    view_count: u64,
}

impl RscPart<RscType>
for StatsData {
    type Backing = StatsData;

    const PART_NAME: &'static str = "statistic";
    const PART_KEY: PartKey = PartKey::Statistics;
}

impl ChannelStats
for StatsData
{
    async fn hide_subs_count(
        &self
    ) -> Result<bool, YtError> {
        Ok(self.hide_subs_count)
    }

    async fn subs_count(
        &self
    ) -> Result<u64, YtError> {
        Ok(self.subs_count)
    }
    
    async fn video_count(
        &self
    ) -> Result<u64, YtError> {
        Ok(self.video_count)
    }

    async fn view_count(
        &self
    ) -> Result<u64, YtError> {
        Ok(self.view_count)
    }
}
