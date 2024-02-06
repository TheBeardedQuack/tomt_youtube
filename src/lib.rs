pub mod auth;
pub mod client;
pub mod error;
pub mod page;
pub mod resource;
pub mod request;

mod id;

#[cfg(test)]
mod test
{
    use crate::{
        auth::YouTubeAuth,
        client::YouTubeClient,
        error::YtError,
        resource::channel::ChannelId,
    };

    #[cfg(debug_assertions)]
    #[tokio::test]
    pub async fn channel_example()
    -> Result<(), YtError> {
        let client = YouTubeClient::new(
            YouTubeAuth::ApiKey(env!("YT_API_KEY").to_string()),
            None
        ).expect("Failed to build client");
        
        let channel = client.channel(ChannelId::new("UC-uE71JAcOCAGbTaS2_C1BQ"));

        let channel_data = channel.with_details()
            .with_snippet()
            .with_statistics()
            .fetch().await?;

        println!("ChannelData = {channel_data:?}");

        let snip = channel.snippet();
        let snippet_data = snip.fetch().await?;

        println!("SnippetData = {snippet_data:?}");
        Ok(())
    }
}

pub(crate) fn stringed_int<'de, D: serde::Deserializer<'de>>(
    deserializer: D
) -> Result<u64, D::Error> {
    struct StringedInt;
    
    impl<'de> serde::de::Visitor<'de>
    for StringedInt
    {
        type Value = u64;
    
        fn expecting(
            &self,
            formatter: &mut std::fmt::Formatter
        ) -> std::fmt::Result {
            write!(formatter, "An unsigned long, or an unsigned long wrapped as a string")
        }
    
        fn visit_u64<E: serde::de::Error>(
            self,
            v: u64
        ) -> Result<Self::Value, E> {
            Ok(v)
        }
    
        fn visit_str<E: serde::de::Error>(
            self,
            v: &str
        ) -> Result<Self::Value, E> {
            v.parse().map_err(|_e| serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &self
            ))
        }
    }
    
    deserializer.deserialize_any(StringedInt)
}
