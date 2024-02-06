mod channel_id;
pub use channel_id::*;

mod playlist_id;
pub use playlist_id::*;

use crate::error::IdError;

use std::ops::Deref;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct StringId(String);

impl StringId
{
    pub fn new<S: AsRef<str>>(
        id: S
    ) -> Result<Self, IdError> {
        let id_str = id.as_ref();

        match id_str.is_empty() {
            true => Err(IdError::Empty),
            false => match id_str.is_ascii() {
                true => Ok(Self(id_str.to_string())),
                false => Err(IdError::Invalid),
            },
        }
    }
}

impl AsRef<str>
for StringId
{
    fn as_ref(
        &self
    ) -> &str {
        self.0.as_ref()
    }
}

impl Deref
for StringId
{
    type Target = <String as Deref>::Target;

    fn deref(
        &self
    ) -> &Self::Target {
        self.0.deref()
    }
}

impl std::fmt::Display
for StringId
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
