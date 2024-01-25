use crate::{
    error::YtError,
    resources::{
        channel::ChannelData as RscType,
        thumbnail::ThumbnailList,
        Resource, RscPart
    }
};

use serde::{Deserialize, Serialize};

type PartKey = <RscType as Resource>::PartKey;

pub trait ChannelSnippet
{
    fn title(
        &self
    ) -> impl std::future::Future<Output = Result<String, YtError>> + Send;

    fn description(
        &self
    ) -> impl std::future::Future<Output = Result<String, YtError>> + Send;

    fn custom_url(
        &self
    ) -> impl std::future::Future<Output = Result<String, YtError>> + Send;
    
    fn thumbnails(
        &self
    ) -> impl std::future::Future<Output = Result<ThumbnailList, YtError>> + Send;
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct SnippetData
{
    pub title: String,
    pub description: String,
    pub curstom_url: String,
    pub thumbnails: ThumbnailList,
}

impl RscPart<RscType>
for SnippetData {
    type Backing = SnippetData;

    const PART_KEY: PartKey = PartKey::Snippet;
    const PART_NAME: &'static str = "snippet";
}

impl ChannelSnippet
for SnippetData
{
    async fn title(
        &self
    ) -> Result<String, YtError> {
        Ok(self.title.clone())
    }

    async fn description(
        &self
    ) -> Result<String, YtError> {
        Ok(self.description.clone())
    }

    async fn custom_url(
        &self
    ) -> Result<String, YtError> {
        Ok(self.curstom_url.clone())
    }

    async fn thumbnails(
        &self
    ) -> Result<ThumbnailList, YtError> {
        Ok(self.thumbnails.clone())
    }
}
