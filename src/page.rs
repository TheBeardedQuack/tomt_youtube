use crate::id::StringId;

use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Debug)]
#[derive(Deserialize)]
pub struct PageInfo
{
    #[serde(rename = "totalResults")]
    pub items_total: isize,

    #[serde(rename = "resultsPerPage")]
    pub items_per_page: isize,
}


#[derive(Clone, Debug)]
#[derive(Hash, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct PageToken(StringId);

impl std::ops::Deref
for PageToken
{
    type Target = StringId;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}
