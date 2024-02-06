mod container;
pub(crate) use container::*;

mod response;
use response::*;

use crate::{
    auth::YouTubeAuth,
    error::{
         ResourceError,
         ResponseError,
         YtError,
         PANIC_LOCK_POISONED,
    },
    resources::{
        channel::*,
        Resource,
    }
};

use reqwest::{
    header::{
        self,
        HeaderMap, HeaderValue
    },
    Url,
};
use std::{
    sync::{Arc, RwLock},
    task::{self, Poll},
};

#[derive(Clone, Debug)]
pub struct YouTubeClient
{
    auth: YouTubeAuth,
    ht_client: reqwest::Client,

    channels: Arc<RwLock<RscContainer<Channel>>>,
}

impl YouTubeClient
{
    const BASE_URL: &'static str = "https://www.googleapis.com/youtube/v3";
    const PANIC_BAD_URL: &'static str = "Failed to parse compile time URL";

    pub fn new(
        auth_mode: YouTubeAuth,
        ht_builder: Option<reqwest::ClientBuilder>
    ) -> Result<Self, YtError> {
        let client = ht_builder.unwrap_or_default()
            .default_headers(HeaderMap::from_iter([
                (header::ACCEPT, HeaderValue::from_static("application/json")),
            ]))
            .referer(true)
            .https_only(true)
            .build()?;

        Ok(Self {
            auth: auth_mode,
            ht_client: client,

            channels: Default::default(),
        })
    }

    async fn fetch_channels(
        &self
    ) -> Result<ResponsePacket<Channel>, YtError> {
        let (backup, url) = {
            let mut url = Url::parse(&format!(
                "{}/{}",
                Self::BASE_URL,
                Channel::RSC_NAME
            )).expect(Self::PANIC_BAD_URL);
            let (backup, ids, parts) = {
                let mut channels_wlock = self.channels.write().expect(PANIC_LOCK_POISONED);

                let ids = channels_wlock.pending_ids()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");

                let parts = channels_wlock.pending_parts()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",");

                let backup = channels_wlock.clone();
                channels_wlock.clear_pending();
                (backup, ids, parts)
            };

            url.query_pairs_mut()
                .append_pair("maxResults", "50")
                .append_pair("id", &ids)
                .append_pair("part", &parts);
        
            match &self.auth {
                YouTubeAuth::ApiKey(key) => url.query_pairs_mut().append_pair("key", key),

                #[allow(unreachable_patterns)]
                _ => todo!("Authentication method not yet supported"),
            };

            (backup, url)
        };

        let text = match self.ht_client.get(url).send().await
        {
            Ok(response) => response.text().await?,
            Err(err) => {
                let mut channels_wlock = self.channels.write().expect(PANIC_LOCK_POISONED);

                for id in backup.pending_ids() {
                    channels_wlock.touch_id(id);
                }
                for part in backup.pending_parts() {
                    channels_wlock.touch_part(*part);
                }

                Err(err)?
            },
        };

        let page = match serde_json::from_str::<ResponsePacket<Channel>>(&text) {
            Ok(page) => page,
            Err(err) => Err(ResourceError::Response(
                ResponseError::JsonDeserialize(err)
            ))?,
        };

        // Merge response data into existing data
        {
            let mut channels_wlock = self.channels.write().expect(PANIC_LOCK_POISONED);
            for item in page.items.iter() {
                channels_wlock.entry(item.id().clone())
                    .or_insert_with(|| Channel::with_id(item.id().clone()))
                    .update(item.clone())
                    .ok();
            }
        }

        Ok(page)
    }
}

impl RscHolder<Channel>
for YouTubeClient
{
    fn touch(
        &self,
        rsc_id: &<Channel as Resource>::Id,
        rsc_part: <Channel as Resource>::PartKey
    ) -> &Self {
        let mut lock = self.channels.write().expect(PANIC_LOCK_POISONED);
        lock.touch_id(rsc_id);
        lock.touch_part(rsc_part);

        self
    }

    async fn fetch(
        &self,
        rsc_id: &<Channel as Resource>::Id
    ) -> Result<Channel, crate::error::YtError> {
        if let Some(channel) = self.get(rsc_id) {
            let current = channel.as_parts();
            
            if self.channels.read()
                .expect(PANIC_LOCK_POISONED)
                .pending_parts()
                .all(|p| current.contains(p))
            {
                // Touch resource so it gets refreshed when the next request NEEDS to go out
                self.touch(rsc_id, ChannelPart::Id);
                return Ok(channel);
            }
        }

        // Touch resource so it gets refreshed when the next request NEEDS to go out
        self.touch(rsc_id, ChannelPart::Id);

        { // Immediately yield to allow other ID's and params to be requested, then issue the actual request
            struct Yield{yielded: bool}
            impl std::future::Future for Yield{
                type Output = ();

                fn poll(
                    mut self: std::pin::Pin<&mut Self>,
                    cx: &mut task::Context<'_>
                ) -> Poll<Self::Output> {
                    match self.yielded {
                        true => Poll::Ready(()),
                        false => {
                            self.yielded = true;
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        },
                    }
                }
            }
            Yield{yielded: false}
        }.await;

         match self.fetch_channels()
            .await?.items
            .into_iter()
            .find(|ch| rsc_id.eq(ch.id()))
        {
            Some(data) => Ok(data),
            None => Err(ResourceError::AccessedPartMissing)?,
        }
    }

    fn get(
        &self,
        rsc_id: &<Channel as Resource>::Id
    ) -> Option<<Channel as Resource>::Backing> {
        self.touch(rsc_id, <Channel as Resource>::PartKey::Id);

        self.channels.read()
            .expect(PANIC_LOCK_POISONED)
            .iter()
            .find_map(|(k, v)| match rsc_id == k {
                true => Some(v.clone()),
                false => None,
            })
    }
}
