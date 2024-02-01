use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct PlaylistId(crate::IdType);

impl PlaylistId
{
    pub fn new<S: AsRef<str>>(
        id: S
    ) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl<T> AsRef<T>
for PlaylistId
where crate::IdType: AsRef<T>
{
    fn as_ref(
        &self
    ) -> &T {
        self.deref().as_ref()
    }
}

impl Deref
for PlaylistId
{
    type Target = crate::IdType;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}
