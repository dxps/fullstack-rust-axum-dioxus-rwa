//! This module provides:
//! - `AppError` - to abstract any infrastructure and low-level errors (like database related ones) into an app (domain) specific ones.
//! - `AppUseCase`s - relevant for the proper conversion from a low-level error to a higher (`AppError`) one.
//!
//! Different cases are considered such as:
//! - for a database error with code 23505 (see its [postgres specifics](https://www.postgresql.org/docs/9.3/errcodes-appendix.html))
use thiserror::Error;

#[derive(Debug)]
pub enum AppUseCase {
    UserRegister,
    UserLogin,
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists")]
    UserRepoSaveEmailAlreadyExistsErr,

    #[error("wrong credentials")]
    LoginWrongCredentialsErr,

    #[error("invalid token")]
    InvalidTokenErr,

    #[error("unknown reason")]
    UnknownErr,
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        log::debug!("From (sqlx err, case): {:?}", ctx);
        // Considering the use case first, then the possible errors within.
        match ctx.1 {
            AppUseCase::UserRegister => match ctx.0.into_database_error() {
                Some(e) => {
                    if let Some(ec) = e.code() {
                        log::debug!("It's a db err with code {ec}");
                        if ec == "23505" {
                            return AppError::UserRepoSaveEmailAlreadyExistsErr;
                        }
                    }
                    AppError::UnknownErr
                }
                None => AppError::UnknownErr,
            },
            AppUseCase::UserLogin => match ctx.0 {
                sqlx::Error::RowNotFound => AppError::LoginWrongCredentialsErr,
                _ => AppError::UnknownErr,
            },
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        log::debug!("From jwt err: {:?}", err);
        AppError::InvalidTokenErr
    }
}
