use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ThumbnailList = HashMap<String, Thumbnail>;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct Thumbnail
{
    pub width: u32,
    pub height: u32,
    pub url: String,
}
