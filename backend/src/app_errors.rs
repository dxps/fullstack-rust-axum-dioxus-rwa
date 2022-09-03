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
    AnyTokenProtectedOperation,
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists")]
    UserRepoSaveEmailAlreadyExistsErr,

    #[error("wrong credentials")]
    LoginWrongCredentialsErr,

    #[error("unauthorized")]
    AuthUnauthorizedErr,

    #[error("invalid token: {0}")]
    InvalidTokenErr(String),

    // #[error("expired token")]
    // TokenExpiredErr,
    #[error("internal error")]
    InternalErr,
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        log::debug!("From (sqlx err, case): {:?}", ctx);
        // Considering the use case first, then the possible errors within.
        match ctx.1 {
            // User Registration case
            AppUseCase::UserRegister => match ctx.0.into_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::UserRepoSaveEmailAlreadyExistsErr,
                        _ => AppError::InternalErr,
                    },
                    None => AppError::InternalErr,
                },
                None => AppError::InternalErr,
            },
            // User Login case
            AppUseCase::UserLogin => match ctx.0 {
                sqlx::Error::RowNotFound => AppError::LoginWrongCredentialsErr,
                _ => AppError::InternalErr,
            },
            // Anything else is treated as an internal error.
            _ => AppError::InternalErr,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        log::debug!("From jwt err: {:?}", err);
        AppError::InvalidTokenErr(err.to_string())
    }
}
