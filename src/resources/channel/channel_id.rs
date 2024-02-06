use crate::resources::{
    channel::Channel as RscType,
    Resource, RscPart,
};

use serde::{Deserialize, Serialize};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug, Default)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct ChannelId(crate::IdType);

impl ChannelId
{
    pub fn new<S: AsRef<str>>(
        id: S
    ) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl<T> AsRef<T>
for ChannelId
where crate::IdType: AsRef<T>
{
    fn as_ref(
        &self
    ) -> &T {
        self.0.as_ref()
    }
}

impl std::ops::Deref
for ChannelId
{
    type Target = crate::IdType;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display
for ChannelId
where crate::IdType:
    std::fmt::Display
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RscPart<RscType>
for ChannelId
{
    type Backing = Self;

    const PART_KEY: <RscType as Resource>::PartKey = PartKey::Id;
    const PART_NAME: &'static str = "id";
}
