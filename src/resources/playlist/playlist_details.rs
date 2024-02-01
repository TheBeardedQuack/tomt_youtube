use serde::Deserialize;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct PlaylistDetails
{
    #[serde(rename = "itemCount")]
    pub video_count: u32,
}
