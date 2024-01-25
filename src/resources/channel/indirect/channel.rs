use std::future::Future;

use crate::{
    client::{RscHolder, YouTubeClient},
    error::YtError,
    resources::{
        channel::{
            indirect::SnippetRef,
            Channel, ChannelId, ChannelParts, ChannelSnippet,
        },
        Resource, RscId,
    },
};

#[derive(Clone, Debug)]
pub struct ChannelRef<'a>
{
    client: &'a YouTubeClient,
    id: ChannelId,
}

impl<'yt> ChannelRef<'yt>
{
    pub fn new(
        client: &'yt YouTubeClient,
        id: ChannelId
    ) -> Self {
        Self{
            client,
            id,
        }
    }

    pub fn client(
        &self
    ) -> &'yt YouTubeClient {
        self.client
    }
}

impl RscId
for ChannelRef<'_>
{
    fn id(
        &self
    ) -> &<Self as Resource>::Id {
        &self.id
    }
}

impl Channel
for ChannelRef<'_> {
    fn snippet(
        &self
    ) -> impl Future<Output = Result<impl ChannelSnippet, YtError>> {
        SnippetRef::from(self.clone())
    }
}

impl ChannelRef<'_>
{
    pub fn with_snippet(
        &self
    ) -> &Self {
        self.client().touch(self.id(), ChannelParts::Snippet);
        self
    }
}
