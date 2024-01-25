use crate::error::YtError;
use super::{Resource, RscId};

pub mod indirect;

mod parts;
pub use parts::*;

mod data;
pub use data::*;

mod details;
pub use details::*;

mod id;
pub use id::*;

mod snippet;
pub use snippet::*;

mod stats;
pub use stats::*;

pub trait Channel:
    RscId +
    Resource<Id = ChannelId, PartKey = ChannelParts, Backing = ChannelData>
{
    fn snippet(
        &self
    ) -> impl std::future::Future<Output = Result<impl ChannelSnippet, YtError>> + Send;

    fn details(
        &self
    ) -> impl std::future::Future<Output = Result<impl ChannelDetails, YtError>> + Send;

    fn stats(
        &self
    ) -> impl std::future::Future<Output = Result<impl ChannelStats, YtError>> + Send;
}

impl<ChT: Channel> Resource
for ChT {
    type Id = ChannelId;
    type PartKey = ChannelParts;
    type Backing = ChannelData;

    const RSC_NAME: &'static str = "channel";
}
