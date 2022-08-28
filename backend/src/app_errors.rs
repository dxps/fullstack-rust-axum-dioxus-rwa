use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Email already exists")]
    UserRepoSaveEmailAlreadyExistsErr,

    #[error("Unknown reason")]
    UserRepoSaveUnknownErr,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        log::debug!("Converting error: {}", e);
        match e.into_database_error() {
            Some(e) => {
                if let Some(ec) = e.code() {
                    // FYI: See https://www.postgresql.org/docs/9.3/errcodes-appendix.html
                    if ec == "23505" {
                        return AppError::UserRepoSaveEmailAlreadyExistsErr;
                    }
                }
                AppError::UserRepoSaveUnknownErr
            }
            None => AppError::UserRepoSaveUnknownErr,
        }
    }
}
