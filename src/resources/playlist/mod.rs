mod playlist_details;
pub use playlist_details::*;

mod playlist_id;
pub use playlist_id::*;

mod playlist_parts;
pub use playlist_parts::*;

mod playlist_snippet;
pub use playlist_snippet::*;

mod playlist_status;
pub use playlist_status::*;

use serde::Deserialize;

use super::Resource;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct Playlist
{
    id: PlaylistId,

    #[serde(rename = "contentDetails")]
    pub details: Option<PlaylistDetails>,

    pub snippet: Option<PlaylistSnippet>,

    pub status: Option<PlaylistStatus>,
}

impl Resource
for Playlist
{
    type Id = PlaylistId;
    type PartKey = PlaylistParts;
    type Backing = Self;

    const RSC_NAME: &'static str = "playlists";

    fn id(
        &self
    ) -> &Self::Id {
        &self.id
    }
}
