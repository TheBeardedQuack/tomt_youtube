use crate::resources::{
    channel::{
        Channel as RscType,
        ChannelParts,
    },
    privacy::Privacy,
    Resource, RscPart,
};

use serde::Deserialize;

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct ChannelStatus
{
    #[serde(rename = "privacyStatus")]
    pub privacy: Privacy
}

impl RscPart<RscType>
for ChannelStatus
{
    type Backing = Self;

    const PART_KEY: PartKey = ChannelParts::Snippet;
    const PART_NAME: &'static str = "status";
}
