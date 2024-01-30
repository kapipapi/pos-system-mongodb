use std::fmt::Display;
use mongodb::bson::Uuid;

#[derive(Debug)]
pub enum RepoError {
    MongoDBError(mongodb::error::Error),

    DotenvError(String),
    DeserializeError(std::fmt::Error),
    CollectionNotFound,

    IdInvalidUuid,
    IdNotFound(Uuid),
}

impl From<mongodb::error::Error> for RepoError {
    fn from(error: mongodb::error::Error) -> Self {
        RepoError::MongoDBError(error)
    }
}

impl Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::MongoDBError(ref error) => write!(f, "MongoDB Error: {}", error),
            RepoError::DotenvError(error_msg) => write!(f, "Error loading environment variables: {}", error_msg),
            RepoError::DeserializeError(error) => write!(f, "JSON deserialization error: {}", error),
            RepoError::CollectionNotFound => write!(f, "Collection not found"),
            RepoError::IdInvalidUuid => write!(f, "Invalid UUID id"),
            RepoError::IdNotFound(id) => write!(f, "Id not found: {}", id),
        }
    }
}