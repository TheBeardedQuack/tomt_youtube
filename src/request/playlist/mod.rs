use crate::{
    request::RscRequest,
    resource::{
        playlist::{Playlist, PlaylistId, PlaylistPart},
        Resource
    },
};

mod into_ids;
pub use into_ids::*;

mod into_parts;
pub use into_parts::*;

#[derive(Clone, Debug)]
pub struct PlaylistRequest
{
    ids: Vec<PlaylistId>,
    parts: Vec<PlaylistPart>,
}

impl PlaylistRequest
{
    pub fn new<
        Ids: IntoIds,
        Parts: IntoParts,
    > (
        id_list: Ids,
        part_list: Option<Parts>
    ) -> Option<Self> {
        let ids: Vec<_> = id_list.to_ids().collect();

        match ids.is_empty() {
            true => None,
            false => {
                let parts = part_list.map(|p| p.to_parts().collect()).unwrap_or_default();
                let mut result = Self{ids, parts};
                result.with_parts(PlaylistPart::Id).clean();

                Some(result)
            }
        }
    }

    fn clean(
        &mut self
    ) -> &mut Self {
        self.ids.sort_by(|a, b| a.cmp(b));
        self.ids.dedup();
        self.ids.shrink_to_fit();

        self.parts.sort_by(|a, b| a.cmp(b));
        self.parts.dedup();
        self.parts.shrink_to_fit();

        self
    }

    pub fn with_ids<Ids: IntoIds>(
        &mut self,
        id_list: Ids
    ) -> &mut Self {
        for id in id_list.to_ids() {
            if !self.ids.contains(&id) {
                self.ids.push(id);
            }
        }
        self
    }

    pub fn with_parts<Parts: IntoParts>(
        &mut self,
        part_list: Parts
    ) -> &mut Self {
        for p in part_list.to_parts() {
            if ! self.parts.contains(&p) {
                self.parts.push(p);
            }
        }
        self
    }

    #[inline]
    pub fn with_details(
        &mut self
    ) -> &mut Self {
        self.with_parts(PlaylistPart::Details)
    }

    #[inline]
    pub fn with_snippet(
        &mut self
    ) -> &mut Self {
        self.with_parts(PlaylistPart::Snippet)
    }
}

impl RscRequest<Playlist>
for PlaylistRequest
{
    fn rsc_name(
        &self
    ) -> &'static str {
        <Playlist as Resource>::RSC_NAME
    }

    fn ids(
        &self
    ) -> &Vec<<Playlist as Resource>::Id> {
        &self.ids
    }

    fn parts(
        &self
    ) -> &Vec<<Playlist as Resource>::PartKey> {
        &self.parts
    }

    fn page(
        &self
    ) -> Option<String> {
        None
    }
}
