use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub enum Privacy
{
    #[serde(rename = "public")]
    Public = 0,
    
    #[serde(rename = "unlisted")]
    Unlisted,

    #[default]
    #[serde(rename = "private")]
    Private,
}
