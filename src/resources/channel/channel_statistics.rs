use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

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

    #[serde(rename = "subscriberCount", deserialize_with = "stringed_int")]
    pub subs_count: u64,

    #[serde(rename = "videoCount", deserialize_with = "stringed_int")]
    pub video_count: u64,

    #[serde(rename = "viewCount", deserialize_with = "stringed_int")]
    pub view_count: u64,
}

impl RscPart<RscType>
for ChannelStats {
    type Backing = ChannelStats;

    const PART_NAME: &'static str = "statistic";
    const PART_KEY: PartKey = PartKey::Statistics;
}

fn stringed_int<'de, D: Deserializer<'de>>(
    deserializer: D
) -> Result<u64, D::Error> {
    struct StringedInt;
    
    impl<'de> Visitor<'de>
    for StringedInt
    {
        type Value = u64;
    
        fn expecting(
            &self,
            formatter: &mut std::fmt::Formatter
        ) -> std::fmt::Result {
            write!(formatter, "An unsigned long, or an unsigned long wrapped as a string")
        }
    
        fn visit_u64<E: serde::de::Error>(
            self,
            v: u64
        ) -> Result<Self::Value, E> {
            Ok(v)
        }
    
        fn visit_str<E: serde::de::Error>(
            self,
            v: &str
        ) -> Result<Self::Value, E> {
            v.parse().map_err(|_e| serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self
            ))
        }
    }
    
    deserializer.deserialize_any(StringedInt)
}
