pub mod auth;
pub mod client;
pub mod error;
pub mod resources;

pub(crate) type IdType = String;

#[cfg(test)]
mod test
{
    use crate::{
        auth::YouTubeAuth,
        client::YouTubeClient,
        error::YtError,
        resources::channel::ChannelId,
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
            .with_stats()
            .fetch().await?;

        println!("ChannelData = {channel_data:?}");

        let snip = channel.snippet();
        let snippet_data = snip.fetch().await?;

        println!("SnippetData = {snippet_data:?}");
        Ok(())
    }
}
