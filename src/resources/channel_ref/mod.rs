mod chanref_details;
pub use chanref_details::*;

mod chanref_snippet;
pub use chanref_snippet::*;

mod chanref_statistics;
pub use chanref_statistics::*;

mod chanref_status;
pub use chanref_status::*;

use crate::{
    client::{RscHolder, YouTubeClient},
    error::YtError,
    resources::{
        channel::{
            Channel,
            ChannelId,
            ChannelParts,
        },
        Resource,
    }
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

    pub async fn fetch(
        &self
    ) -> Result<Channel, YtError> {
        self.client.touch(self.id(), ChannelParts::Id);
        self.client.fetch(self.id()).await
    }
}

impl Resource
for ChannelRef<'_>
{
    type Id = ChannelId;
    type PartKey = ChannelParts;
    type Backing = Channel;

    const RSC_NAME: &'static str = "channels";

    fn id(
        &self
    ) -> &<Self as Resource>::Id {
        &self.id
    }
}

impl ChannelRef<'_> {
    pub fn snippet(
        &self
    ) -> SnippetRef {
        SnippetRef::from(self.clone())
    }

    pub fn details(
        &self
    ) -> DetailsRef {
        DetailsRef::from(self.clone())
    }

    pub fn stats(
        &self
    ) -> StatsRef {
        StatsRef::from(self.clone())
    }

    pub fn with_snippet(
        &self
    ) -> &Self {
        self.client().touch(self.id(), ChannelParts::Snippet);
        self
    }

    pub fn with_details(
        &self
    ) -> &Self {
        self.client().touch(self.id(), ChannelParts::Details);
        self
    }

    pub fn with_stats(
        &self
    ) -> &Self {
        self.client().touch(self.id(), ChannelParts::Statistics);
        self
    }
}
