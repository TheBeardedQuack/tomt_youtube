use crate::request::channel::ChannelId;

pub trait IntoIds
{
    fn to_ids(
        self
    ) -> Vec<ChannelId>;
}

impl<T: Into<Vec<ChannelId>>> IntoIds
for T
{
    fn to_ids(
        self
    ) -> Vec<ChannelId> {
        self.into()
    }
}

impl From<ChannelId>
for Vec<ChannelId>
{
    fn from(
        value: ChannelId
    ) -> Self {
        vec![value]
    }
}
