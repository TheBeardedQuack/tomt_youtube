use crate::{
    error::ResourceError,
    resources::{Resource, RscId},
};

use super::{
    Channel, ChannelDetails, ChannelId, ChannelParts, ChannelSnippet, ChannelStats, DetailsData, SnippetData, StatsData
};

#[derive(Clone, Debug)]
pub struct ChannelData
{
    pub id: ChannelId,
    pub snippet: Option<SnippetData>,
    pub details: Option<DetailsData>,
    pub stats: Option<StatsData>,
}

impl ChannelData
{
    pub fn as_parts(
        &self
    ) -> Vec<<Self as Resource>::PartKey> {
        [
            Some(ChannelParts::Id),
            self.snippet.as_ref().map(|_| ChannelParts::Snippet),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Channel
for ChannelData
{
    async fn snippet(
        &self
    ) -> Result<impl ChannelSnippet, crate::error::YtError> {
        match &self.snippet {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }

    async fn details(
        &self
    ) -> Result<impl ChannelDetails, crate::error::YtError> {
        match &self.details {
            Some(deets) => Ok(deets.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }

    async fn stats(
        &self
    ) -> Result<impl ChannelStats, crate::error::YtError> {
        match &self.stats {
            Some(stats) => Ok(stats.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl RscId
for ChannelData
{
    fn id(
        &self
    ) -> &Self::Id {
        &self.id
    }
}
