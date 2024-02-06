use crate::{
    resources::{
        channel::{
            Channel,
            ChannelId,
            ChannelPart,
        },
        Resource,
    },
    request::RscRequest,
};

mod into_ids;
pub use into_ids::*;

mod into_parts;
pub use into_parts::*;

#[derive(Clone, Debug)]
pub struct ChannelRequest
{
    ids: Vec<ChannelId>,
    parts: Vec<ChannelPart>,
}

impl ChannelRequest
{
    pub fn new<
        Ids: IntoIds,
        Parts: IntoParts,
    > (
        id_list: Ids,
        part_list: Option<Parts>
    ) -> Option<Self> {
        let ids = id_list.to_ids();

        match ids.is_empty() {
            true => None,
            false => {
                let parts = part_list.map(|p| p.to_parts().collect()).unwrap_or_default();
                let mut result = Self{ids, parts};
                result.with_parts(ChannelPart::Id).clean();

                Some(result)
            }
        }
    }

    fn clean(
        &mut self
    ) -> &mut Self {
        self.ids.sort_by(|a, b| a.cmp(b));
        self.ids.dedup();

        self.parts.sort_by(|a, b| a.cmp(b));
        self.parts.dedup();

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
        self.with_parts(ChannelPart::Details)
    }

    #[inline]
    pub fn with_snippet(
        &mut self
    ) -> &mut Self {
        self.with_parts(ChannelPart::Snippet)
    }

    #[inline]
    pub fn with_statistics(
        &mut self
    ) -> &mut Self {
        self.with_parts(ChannelPart::Statistics)
    }
}

impl RscRequest<Channel>
for ChannelRequest
{
    fn rsc_name(
        &self
    ) -> &'static str {
        <Channel as Resource>::RSC_NAME
    }

    fn ids(
        &self
    ) -> &Vec<<Channel as Resource>::Id> {
        &self.ids
    }

    fn parts(
        &self
    ) -> &Vec<<Channel as Resource>::PartKey> {
        &self.parts
    }
}
