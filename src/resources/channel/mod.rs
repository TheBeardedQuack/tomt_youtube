use crate::error::YtError;
use super::{Resource, RscId};

pub mod indirect;

mod channel_parts;
pub use channel_parts::*;

mod data;
pub use data::*;

mod details;
pub use details::*;

mod id;
pub use id::*;

mod snippet;
pub use snippet::*;

pub trait Channel:
    RscId +
    Resource<Id = ChannelId, PartKey = ChannelParts, Backing = ChannelData>
{
    fn snippet(
        &self
    ) -> impl std::future::Future<Output = Result<impl ChannelSnippet, YtError>> + Send;
}

impl<ChT: Channel> Resource
for ChT {
    type Id = ChannelId;
    type PartKey = ChannelParts;
    type Backing = ChannelData;

    const RSC_NAME: &'static str = "channel";
}
