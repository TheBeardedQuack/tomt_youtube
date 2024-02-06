use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    error::YtError,
    resources::Resource,
};

pub trait RscHolder<R: Resource>
{
    fn touch(
        &self,
        rsc_id: &R::Id,
        rsc_part: R::PartKey
    ) -> &Self;

    async fn fetch(
        &self,
        rsc_id: &R::Id
    ) -> Result<R, YtError>;

    fn get(
        &self,
        rsc_id: &R::Id
    ) -> Option<R::Backing>;
}

// For this site each list will be relatively small, just using a vec.
// We can rip through the array when inserting to prevent dupes
#[derive(Clone, Debug)]
pub struct RscContainer<R: Resource>
{
    items: HashMap<R::Id, R>,
    pending_ids: Vec<R::Id>,
    pending_parts: Vec<R::PartKey>,
}

impl<R: Resource> Default
for RscContainer<R>
{
    fn default(
        // no args
    ) -> Self {
        Self {
            items: Default::default(),
            pending_ids: Default::default(),
            pending_parts: Default::default()
        }
    }
}

impl<R: Resource>
RscContainer<R>
{
    pub fn touch_id(
        &mut self,
        rsc_id: &R::Id
    ) -> &Self {
        if ! self.pending_ids.contains(rsc_id) {
            self.pending_ids.push(rsc_id.clone());
        }
        self
    }

    pub fn touch_part(
        &mut self,
        rsc_part: R::PartKey
    ) -> &Self {
        if ! self.pending_parts.contains(&rsc_part) {
            self.pending_parts.push(rsc_part);
        }
        self
    }

    pub fn pending_ids(
        &self
    ) -> impl Iterator<Item = &R::Id> {
        self.pending_ids.iter()
    }

    pub fn pending_parts(
        &self
    ) -> impl Iterator<Item = &R::PartKey> {
        self.pending_parts.iter()
    }

    pub fn clear_pending(
        &mut self
    ) -> &mut Self {
        self.pending_ids.clear();
        self.pending_parts.clear();
        self
    }
}

impl<R: Resource> Deref
for RscContainer<R>
{
    type Target = HashMap<R::Id, R>;

    fn deref(
        &self
    ) -> &Self::Target {
        &self.items
    }
}

impl<R: Resource> DerefMut
for RscContainer<R>
{
    fn deref_mut(
        &mut self
    ) -> &mut Self::Target {
        &mut self.items
    }
}
