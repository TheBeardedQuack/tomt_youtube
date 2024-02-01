use crate::resources::Resource;
use serde::Deserialize;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct ResponsePacket<DataT: Resource>
{
    pub items: Vec<DataT>,

    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,

    #[serde(rename = "nextPageToken")]
    pub next_page: Option<String>,

    #[serde(rename = "prevPageToken")]
    pub prev_page: Option<String>,
}

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct PageInfo
{
    #[serde(rename = "totalResults")]
    pub items_total: isize,

    #[serde(rename = "resultsPerPage")]
    pub items_per_page: isize,
}
