use crate::resources::privacy::Privacy;

use serde::Deserialize;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct ChannelStatus
{
    #[serde(rename = "privacyStatus")]
    pub privacy: Privacy
}
