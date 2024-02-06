use crate::{
    error::IdError, id::StringId, resource::{
        playlist::Playlist as RscType,
        Resource, RscPart,
    }
};

use serde::{Deserialize, Serialize};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct PlaylistId(StringId);

impl PlaylistId
{
    pub fn new<S: AsRef<str>>(
        id: S
    ) -> Result<Self, IdError> {
        Ok(Self(StringId::new(id)?))
    }
}

impl From<StringId>
for PlaylistId
{
    fn from(
        value: StringId
    ) -> Self {
        Self(value)
    }
}

impl std::ops::Deref
for PlaylistId
{
    type Target = StringId;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}

impl RscPart<RscType>
for PlaylistId
{
    type Backing = Self;

    const PART_KEY: <RscType as Resource>::PartKey = PartKey::Id;
    const PART_NAME: &'static str = "id";
}
