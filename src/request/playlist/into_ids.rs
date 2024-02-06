use super::PlaylistId;

pub trait IntoIds
{
    fn to_ids(
        self
    ) -> impl Iterator<Item = PlaylistId>;
}

impl<IdIter: Iterator<Item = PlaylistId>>
IntoIds for IdIter
{
    fn to_ids(
        self
    ) -> impl Iterator<Item = PlaylistId> {
        self.into_iter()
    }
}


impl IntoIds
for PlaylistId {
    fn to_ids(
        self
    ) -> impl Iterator<Item = PlaylistId> {
        std::iter::once(self)
    }
}
