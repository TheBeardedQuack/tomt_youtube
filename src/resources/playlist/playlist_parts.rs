use serde::Deserialize;

#[derive(Clone, Copy, Debug)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize)]
pub enum PlaylistParts
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
for PlaylistParts
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        match self {
            PlaylistParts::Id => write!(f, "id"),
            PlaylistParts::Details => write!(f, "contentDetails"),
            PlaylistParts::Snippet => write!(f, "snippet"),
            PlaylistParts::Status => write!(f, "status"),
        }
    }
}
