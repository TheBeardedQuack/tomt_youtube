use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            Channel as RscType,
            ChannelSnippet,
        },
        channel_ref::ChannelRef,
        thumbnail::ThumbnailList,
        Resource, RscPart,
    },
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct SnippetRef<'yt>(ChannelRef<'yt>);

impl<'yt> SnippetRef<'yt>
{
    pub async fn title(
        &self
    ) -> Result<String, YtError> {
        let snip = self.fetch().await?;
        Ok(snip.title)
    }

    pub async fn description(
        &self
    ) -> Result<String, YtError> {
        let snip = self.fetch().await?;
        Ok(snip.description)
    }

    pub async fn custom_url(
        &self
    ) -> Result<String, YtError> {
        let snip = self.fetch().await?;
        Ok(snip.custom_url)
    }

    pub async fn thumbnails(
        &self
    ) -> Result<ThumbnailList, YtError> {
        let snip = self.fetch().await?;
        Ok(snip.thumbnails)
    }

    pub async fn fetch(
        &self
    ) -> Result<ChannelSnippet, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.snippet {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl RscPart<RscType>
for SnippetRef<'_> {
    type Backing = ChannelSnippet;

    const PART_NAME: &'static str = "snippet";
    const PART_KEY: PartKey = PartKey::Snippet;
}

impl<'yt> From<ChannelRef<'yt>>
for SnippetRef<'yt>
{
    fn from(
        value: ChannelRef<'yt>
    ) -> Self {
        Self(value)
    }
}
