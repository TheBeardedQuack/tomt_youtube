mod container;

use std::{
    sync::RwLock,
    task::Poll,
};

pub(crate) use container::*;

use crate::{
    auth::YouTubeAuth,
    error::{self, ResourceError, YtError},
    resources::{
        channel::*,
        Resource,
    }
};

use self::indirect::ChannelRef;

#[derive(Debug)]
pub struct YouTubeClient
{
    auth: YouTubeAuth,
    channels: RwLock<RscContainer<ChannelData>>,
}

impl YouTubeClient
{
    pub fn new(
        auth_mode: YouTubeAuth
    ) -> Self {
        Self {
            auth: auth_mode,
            channels: Default::default(),
        }
    }

    pub fn channel(
        &self,
        id: ChannelId
    ) -> impl Channel + '_ {
        ChannelRef::new(self, id)
    }

    async fn fetch_channels(
        &self
    ) -> Result<Vec<ChannelData>, YtError> {
        let _result = Vec::<ChannelData>::new();

        todo!("Issue HTTP request");

        todo!("Parse data out of response");

        todo!("Merge response data into existing data");

        todo!("Return only the newly requested data");

        Ok(_result)
    }
}

impl RscHolder<ChannelData>
for YouTubeClient
{
    fn touch(
        &self,
        rsc_id: &<ChannelData as Resource>::Id,
        rsc_part: <ChannelData as Resource>::PartKey
    ) -> &Self {
        let mut lock = self.channels.write().expect(error::PANIC_LOCK_POISONED);
        lock.touch_id(rsc_id);
        lock.touch_part(rsc_part);

        self
    }

    async fn fetch(
        &self,
        rsc_id: &<ChannelData as Resource>::Id
    ) -> Result<ChannelData, crate::error::YtError> {
        if let Some(channel) = self.get(rsc_id) {
            let current = channel.as_parts();
            if self.channels.read()
                .expect(error::PANIC_LOCK_POISONED)
                .pending_parts()
                .all(|p| current.contains(p))
            {
                // Touch resource so it gets refreshed when the next request NEEDS to go out
                self.touch(rsc_id, ChannelParts::Id);
                return Ok(channel);
            }
        }

        { // Immediately yield to allow other ID's and params to be requested, then issue the actual request
            // Touch resource so it gets refreshed when the next request NEEDS to go out
            self.touch(rsc_id, ChannelParts::Id);

            let mut init = false;
            std::future::poll_fn(move |_| {
                match init {
                    false => {
                        init = true;
                        Poll::Pending
                    },
                    true => Poll::Ready(())
                }
            })
        }.await;

        match self.fetch_channels().await?
            .into_iter()
            .find(|ch| ch.id.eq(rsc_id))
        {
            Some(data) => Ok(data),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }

    fn get(
        &self,
        rsc_id: &<ChannelData as Resource>::Id
    ) -> Option<<ChannelData as Resource>::Backing> {
        self.touch(rsc_id, <ChannelData as Resource>::PartKey::Id);

        self.channels.read()
            .expect(error::PANIC_LOCK_POISONED)
            .iter()
            .find_map(|(k, v)| match rsc_id == k {
                true => Some(v.clone()),
                false => None,
            })
    }
}
