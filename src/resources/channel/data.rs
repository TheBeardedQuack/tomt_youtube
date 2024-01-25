use crate::{
    error::ResourceError,
    resources::{Resource, RscId},
};

use super::{
    Channel, ChannelId, ChannelParts,
    ChannelSnippet, SnippetData
};

#[derive(Clone, Debug)]
#[derive(PartialEq)]
pub struct ChannelData
{
    pub id: ChannelId,
    pub snippet: Option<SnippetData>,
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
