use crate::{error::YtError, resources::{Resource, RscPart}};

use serde::{Deserialize, Serialize};

use super::ChannelData;

pub trait ChannelSnippet
{
    fn title(
        &self
    ) -> impl std::future::Future<Output = Result<String, YtError>> + Send;

    fn description(
        &self
    ) -> impl std::future::Future<Output = Result<String, YtError>> + Send;

    fn curstom_url(
        &self
    ) -> impl std::future::Future<Output = Result<String, YtError>> + Send;
}

impl<SnipT: ChannelSnippet> RscPart
for SnipT {
    type ForRsc = ChannelData;
    type Backing = SnippetData;

    const PART_KEY: <Self::ForRsc as Resource>::PartKey =
        <Self::ForRsc as Resource>::PartKey::Snippet;

    const PART_NAME: &'static str = "snippet";
}

#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct SnippetData
{
    pub title: String,
    pub description: String,
    pub curstom_url: String,
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

    async fn curstom_url(
        &self
    ) -> Result<String, YtError> {
        Ok(self.curstom_url.clone())
    }
}
