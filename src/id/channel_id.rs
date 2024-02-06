use crate::{
    id::StringId,
    resource::{
        channel::Channel as RscType,
        Resource, RscPart,
    },
};

use serde::{Deserialize, Serialize};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct ChannelId(StringId);

impl std::ops::Deref
for ChannelId
{
    type Target = crate::id::StringId;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}

impl RscPart<RscType>
for ChannelId
{
    type Backing = Self;

    const PART_KEY: <RscType as Resource>::PartKey = PartKey::Id;
    const PART_NAME: &'static str = "id";
}
