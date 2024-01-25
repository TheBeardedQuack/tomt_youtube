use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
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
