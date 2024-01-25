use std::ops::Deref;

#[derive(Clone, Debug, Default)]
#[derive(Hash, PartialEq)]
pub struct ChannelId(crate::IdType);

impl<T> AsRef<T>
for ChannelId
where crate::IdType: AsRef<T>
{
    fn as_ref(
        &self
    ) -> &T {
        self.deref().as_ref()
    }
}

impl Deref
for ChannelId
{
    type Target = crate::IdType;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.0
    }
}
