use crate::{
    resource::Resource,
    page::{PageInfo, PageToken},
};
use serde::Deserialize;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct ResponsePacket<DataT: Resource>
{
    pub items: Vec<DataT>,

    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,

    #[serde(rename = "nextPageToken")]
    pub next_page: Option<PageToken>,

    #[serde(rename = "prevPageToken")]
    pub prev_page: Option<PageToken>,
}
