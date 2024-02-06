mod chan_details_req;
pub use chan_details_req::*;

mod chan_snippet_req;
pub use chan_snippet_req::*;

mod chan_statistics_req;
pub use chan_statistics_req::*;

mod chan_status_req;
pub use chan_status_req::*;

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
pub struct ChannelRequest<'a>
{
    client: &'a YouTubeClient,
    id: ChannelId,
}

impl<'yt> ChannelRequest<'yt>
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
for ChannelRequest<'_>
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

impl ChannelRequest<'_> {
    pub fn snippet(
        &self
    ) -> SnippetRequest {
        SnippetRequest::from(self.clone())
    }

    pub fn details(
        &self
    ) -> DetailsRequest {
        DetailsRequest::from(self.clone())
    }

    pub fn stats(
        &self
    ) -> StatsRequest {
        StatsRequest::from(self.clone())
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
