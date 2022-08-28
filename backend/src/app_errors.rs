use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to save user: `{0}`")]
    UserRepoSaveErr(String),
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        let msg = e.into_database_error().unwrap().message().to_string();
        AppError::UserRepoSaveErr(msg)
    }
}
