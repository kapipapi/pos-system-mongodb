#[derive(Debug)]
pub enum RepoError {
    NotFound(String),
    InternalServerError(String),
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RepoError::NotFound(str) => write!(f, "Not Found Error: {str}"),
            RepoError::InternalServerError(str) => write!(f, "Internal Server Error: {str}"),
        }
    }
}

impl std::error::Error for RepoError {}