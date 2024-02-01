use std::fmt::Display;

use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Hash)]
#[derive(Deserialize)]
pub enum ChannelParts
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
for ChannelParts
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        match self {
            ChannelParts::Id => write!(f, "id"),
            ChannelParts::Snippet => write!(f, "snippet"),
            ChannelParts::Details => write!(f, "contentDetails"),
            ChannelParts::Statistics => write!(f, "statistics"),
            ChannelParts::Status => write!(f, "status"),
        }
    }
}
