use crate::resource::{
    channel::{
        Channel as RscType,
        ChannelPart,
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

    const PART_KEY: PartKey = ChannelPart::Snippet;
    const PART_NAME: &'static str = "status";
}
