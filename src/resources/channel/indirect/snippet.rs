use crate::{
    client::RscHolder,
    error::{ResourceError, YtError},
    resources::{
        channel::{
            indirect::ChannelRef,
            ChannelData as RscType,
            ChannelSnippet, SnippetData
        },
        thumbnail::ThumbnailList,
        Resource, RscId, RscPart,
    },
};

use std::{
    future::Future,
    pin::pin,
    task::{ready, Poll},
};

type PartKey = <RscType as Resource>::PartKey;

#[derive(Clone, Debug)]
pub struct SnippetRef<'yt>(ChannelRef<'yt>);

impl RscPart<RscType>
for SnippetRef<'_> {
    type Backing = SnippetData;

    const PART_NAME: &'static str = "snippet";
    const PART_KEY: PartKey = PartKey::Snippet;
}

impl ChannelSnippet
for SnippetRef<'_>
{
    async fn title(
        &self
    ) -> Result<String, YtError> {
        self.clone().await?
            .title().await
    }

    async fn description(
        &self
    ) -> Result<String, YtError> {
        self.clone().await?
            .description().await
    }

    async fn custom_url(
        &self
    ) -> Result<String, YtError> {
        self.clone().await?
            .custom_url().await
    }

    async fn thumbnails(
        &self
    ) -> Result<ThumbnailList, YtError> {
        self.clone().await?
            .thumbnails().await
    }
}

impl<'yt> SnippetRef<'yt>
{
    async fn run(
        &self
    ) -> Result<SnippetData, YtError> {
        self.0.client().touch(self.0.id(), Self::PART_KEY);

        let chan_data = self.0.client().fetch(self.0.id()).await?;
        match &chan_data.snippet {
            Some(snip) => Ok(snip.clone()),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }
}

impl Future
for SnippetRef<'_>
{
    type Output = Result<SnippetData, YtError>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>
    ) -> Poll<Self::Output> {
        Poll::Ready(ready!(pin!(self.run()).poll(cx)))
    }
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
