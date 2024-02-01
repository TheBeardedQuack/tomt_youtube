use serde::Deserialize;

use crate::resources::{
    channel::Channel as RscType,
    Resource, RscPart
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Deserialize)]
pub struct ChannelStats
{
    #[serde(rename = "hiddenSubscriberCount")]
    pub hide_subs_count: bool,

    #[serde(rename = "subscriberCount", deserialize_with = "crate::stringed_int")]
    pub subs_count: u64,

    #[serde(rename = "videoCount", deserialize_with = "crate::stringed_int")]
    pub video_count: u64,

    #[serde(rename = "viewCount", deserialize_with = "crate::stringed_int")]
    pub view_count: u64,
}

impl RscPart<RscType>
for ChannelStats {
    type Backing = ChannelStats;

    const PART_NAME: &'static str = "statistic";
    const PART_KEY: PartKey = PartKey::Statistics;
}
