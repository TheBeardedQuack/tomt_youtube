mod channel_details;
pub use channel_details::*;

mod channel_id;
pub use channel_id::*;

mod channel_parts;
pub use channel_parts::*;

mod channel_snippet;
pub use channel_snippet::*;

mod channel_statistics;
pub use channel_statistics::*;

mod channel_status;
pub use channel_status::*;

use super::Resource;
use serde::Deserialize;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct Channel
{
    id: ChannelId,

    #[serde(rename = "contentDetails")]
    pub details: Option<ChannelDetails>,

    pub snippet: Option<ChannelSnippet>,

    #[serde(rename = "statistics")]
    pub stats: Option<ChannelStats>,

    pub status: Option<ChannelStatus>,
}

impl Channel
{
    pub fn with_id(
        id: ChannelId
    ) -> Self {
        Self {
            id,
            details: Default::default(),
            snippet: Default::default(),
            stats: Default::default(),
            status: Default::default(),
        }
    }

    pub fn as_parts(
        &self
    ) -> Vec<<Self as Resource>::PartKey> {
        [
            Some(ChannelParts::Id),
            self.details.as_ref().map(|_| ChannelParts::Details),
            self.snippet.as_ref().map(|_| ChannelParts::Snippet),
            self.stats.as_ref().map(|_| ChannelParts::Statistics),
            self.status.as_ref().map(|_| ChannelParts::Status),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// Will only fail if ID's do not match
    /// Returned item is a mutable ref to self, wrapped in Result to indicate if ID's matched
    pub fn update(
        &mut self,
        rhs: Self
    ) -> Result<&mut Self, &mut Self> {
        if self.id() != rhs.id() {
            Err(self)
        }
        else {
            self.details = rhs.details.or(self.details.take());
            self.snippet = rhs.snippet.or(self.snippet.take());
            self.stats = rhs.stats.or(self.stats.take());
            self.status = rhs.status.or(self.status.take());

            Ok(self)
        }
    }
}

impl Resource
for Channel
{
    type Id = ChannelId;
    type PartKey = ChannelParts;
    type Backing = Self;

    const RSC_NAME: &'static str = "channels";

    fn id(
        &self
    ) -> &Self::Id {
        &self.id
    }
}
