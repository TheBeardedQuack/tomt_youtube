use std::fmt::Display;

use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Hash)]
#[derive(Deserialize)]
pub enum ChannelPart
{
    #[serde(rename = "id")]
    Id,

    #[serde(rename = "snippet")]
    Snippet,

    #[serde(rename = "contentDetails")]
    Details,

    #[serde(rename = "statistics")]
    Statistics,

    #[serde(rename = "status")]
    Status,
}

impl Display
for ChannelPart
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl AsRef<str>
for ChannelPart
{
    fn as_ref(
        &self
    ) -> &'static str {
        match self {
            ChannelPart::Id => "id",
            ChannelPart::Snippet => "snippet",
            ChannelPart::Details => "contentDetails",
            ChannelPart::Statistics => "statistics",
            ChannelPart::Status => "status",
        }
    }
}

impl std::ops::Deref
for ChannelPart
{
    type Target = str;

    fn deref(
        &self
    ) -> &Self::Target {
        self.as_ref()
    }
}
