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
    UpdateUser,
    GetUserProfile,
    FollowUser,
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("email already exists")]
    RegistrationEmailAlreadyExists,

    #[error("wrong credentials")]
    AuthLoginFailed,

    #[error("unauthorized")]
    AuthUnauthorized,

    #[error("invalid token: {0}")]
    AuthInvalidTokenErr(String),

    #[error("invalid input")]
    AuthInvalidInput,

    #[error("entry not found")]
    NothingFound,

    #[error("internal error")]
    InternalErr,

    #[error("")]
    Ignorable,
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        log::debug!("From (sqlx err, case): {:?}", ctx);
        let err = ctx.0;
        // Considering the use case first, then the possible errors within.
        match ctx.1 {
            AppUseCase::UserRegister => match &err.into_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::RegistrationEmailAlreadyExists,
                        _ => AppError::InternalErr,
                    },
                    None => AppError::InternalErr,
                },
                None => AppError::InternalErr,
            },

            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::AuthLoginFailed,
                _ => AppError::InternalErr,
            },

            AppUseCase::GetUserProfile => match &err {
                sqlx::Error::RowNotFound => AppError::NothingFound,
                _ => AppError::InternalErr,
            },

            AppUseCase::FollowUser => match &err.into_database_error() {
                Some(dbe) => match dbe.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::Ignorable,
                        _ => AppError::InternalErr,
                    },
                    None => AppError::InternalErr,
                },
                None => AppError::InternalErr,
            },

            // Anything else is treated as an internal error.
            _ => AppError::InternalErr,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        log::debug!("From sqlx err: {:?}", err);
        AppError::InternalErr
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::AuthInvalidTokenErr(err.to_string())
    }
}
