use std::fmt::{Display, Formatter};
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use crate::repo::error::RepoError;

#[derive(Debug)]
pub enum ServiceError {
    InternalError(String),
    BadRequest(String),
    NotFound(String),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::InternalError(err) => write!(f, "Internal Server Error: {err}"),
            ServiceError::BadRequest(err) => write!(f, "Bad Request: {err}"),
            ServiceError::NotFound(err) => write!(f, "Not Found: {err}"),
        }
    }
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

impl From<RepoError> for ServiceError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::MongoDBError(err) => ServiceError::InternalError(err.to_string()),
            RepoError::DotenvError(err) => ServiceError::InternalError(err),
            RepoError::DeserializeError(err) => ServiceError::InternalError(err.to_string()),
            RepoError::CollectionNotFound => ServiceError::InternalError("Collection not found".to_string()),
            RepoError::IdInvalidUuid => ServiceError::InternalError("Invalid UUID id".to_string()),
            RepoError::IdNotFound(id) => ServiceError::NotFound(format!("Id not found: {}", id)),
            RepoError::BsonSerializationError(err) => ServiceError::InternalError(err.to_string()),
            RepoError::IdsNotFound(ids) => ServiceError::NotFound(format!("Ids not found: {:?}", ids)),
        }
    }
}
