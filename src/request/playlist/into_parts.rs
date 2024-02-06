use super::PlaylistPart;

pub trait IntoParts
{
    fn to_parts(
        self
    ) -> impl Iterator<Item = PlaylistPart>;
}

impl<PartIter: Iterator<Item = PlaylistPart>>
IntoParts for PartIter
{
    fn to_parts(
        self
    ) -> impl Iterator<Item = PlaylistPart> {
        self.into_iter()
    }
}

impl IntoParts
for PlaylistPart {
    fn to_parts(
        self
    ) -> impl Iterator<Item = PlaylistPart> {
        std::iter::once(self)
    }
}
