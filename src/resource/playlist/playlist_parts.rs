use std::ops::Deref;

use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize)]
pub enum PlaylistPart
{
    #[serde(rename = "id")]
    Id,

    #[serde(rename = "contentDetails")]
    Details,

    #[serde(rename = "snippet")]
    Snippet,

    #[serde(rename = "status")]
    Status,
}

impl std::fmt::Display
for PlaylistPart
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl Deref
for PlaylistPart
{
    type Target = str;

    fn deref(
        &self
    ) -> &'static Self::Target {
        match self {
            PlaylistPart::Id => "id",
            PlaylistPart::Details => "contentDetails",
            PlaylistPart::Snippet => "snippet",
            PlaylistPart::Status => "status",
        }
    }
}