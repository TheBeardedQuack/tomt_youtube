pub(crate) const PANIC_LOCK_POISONED: &str = "Lock poisoned";

#[derive(Debug)]
#[derive(thiserror::Error)]
#[error("Error occured in library 'tomt_youtube'.\n{}", self)]
pub enum YtError
{
    Resource(#[from] ResourceError),
    Query(#[from] QueryError),

    #[error("{}", *self)]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug)]
#[derive(thiserror::Error)]
#[error("Error occured on resource object.\n{}", self)]
pub enum ResourceError
{
    #[error("Attempted to access a resources field, which has not been set or fetched (treat as None)")]
    AccessedPartMissing,

    Query(#[from] QueryError),
    Response(#[from] ResponseError),
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum QueryError
{
    #[error("One or more required fields to build a valid query are missing")]
    MissingParams,

    #[error("The provided object has too many populated fields reasonably determine what query to build")]
    TooManyParams
}

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum ResponseError
{
    #[error("Failed to parse JSON from response body: {}", self)]
    JsonDeserialize(#[from] serde_json::Error),
}
