use crate::request::channel::ChannelPart;

pub trait IntoParts
{
    fn to_parts(
        self
    ) -> impl Iterator<Item = ChannelPart>;
}

impl<PartIter: Iterator<Item = ChannelPart>>
IntoParts for PartIter
{
    fn to_parts(
        self
    ) -> impl Iterator<Item = ChannelPart> {
        self.into_iter()
    }
}

impl IntoParts
for ChannelPart {
    fn to_parts(
        self
    ) -> impl Iterator<Item = ChannelPart> {
        std::iter::once(self)
    }
}
