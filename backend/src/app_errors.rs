//! This module provides:
//! - `AppError` - to abstract any infrastructure and low-level errors (like database related ones) into an app (domain) specific ones.
//! - `AppUseCase`s - relevant for the proper conversion from a low-level error to a higher (`AppError`) one.
//!
//! Different cases are considered such as:
//! - for a database error with code 23505 (see its [postgres specifics](https://www.postgresql.org/docs/9.3/errcodes-appendix.html))

use thiserror::Error;

#[derive(Debug)]
pub enum AppUseCase {
    UserRegistration,
    UserLogin,
    AnyTokenProtectedOperation,
    UpdateUser,
    GetUserProfile,
    FollowUser,
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    //
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error("")]
    Ignorable,

    #[error("internal error")]
    InternalErr,

    #[error("invalid request: {0}")]
    InvalidRequest(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),
}

impl From<(sqlx::Error, AppUseCase)> for AppError {
    //
    fn from(ctx: (sqlx::Error, AppUseCase)) -> Self {
        log::debug!("from((sqlx::Error, AppUseCase)): ctx={:?}", ctx);
        let err = ctx.0;
        // Start with the use case as the context, and then cover the possible errors within.
        match ctx.1 {
            AppUseCase::UserRegistration => match &err.into_database_error() {
                Some(e) => match e.code() {
                    Some(code) => match code.as_ref() {
                        "23505" => AppError::AlreadyExists("email".into()),
                        _ => AppError::InternalErr,
                    },
                    None => AppError::InternalErr,
                },
                None => AppError::InternalErr,
            },

            AppUseCase::UserLogin => match &err {
                sqlx::Error::RowNotFound => AppError::Unauthorized("wrong credentials".into()),
                _ => AppError::InternalErr,
            },

            AppUseCase::GetUserProfile => match &err {
                sqlx::Error::RowNotFound => AppError::NotFound("profile".into()),
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
    //
    fn from(err: sqlx::Error) -> Self {
        let mut app_err = AppError::Ignorable;
        log::debug!("from(sqlx:Error): err={:?}", err);
        if err.as_database_error().is_some() {
            // TODO: For now, any db error is classified as internal error.
            app_err = AppError::InternalErr
        }
        app_err
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    //
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized(err.to_string())
    }
}
