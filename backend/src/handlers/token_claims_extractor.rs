use axum::{
    async_trait,
    extract::{TypedHeader, FromRequestParts},
    headers::Authorization, http::{request::Parts},
};

use crate::{
    domain::model::UserId,
    token::{self, Claims, Token},
    AppError,
};

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts, state: &S
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization (HTTP request) header having the value of "Bearer <token>".
        let token = TypedHeader::<Authorization<Token>>::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                log::debug!("Failed to extract the token: {}", err);
                AppError::AuthUnauthorized
            })?;

        match token::verify_jwt(token.0 .0.token()) {
            Ok(claims) => {
                // We just extract and provide the user (aka subject or JWT's `sub`) id.
                Ok(claims.sub.into())
            }
            Err(err) => {
                log::debug!("Token verification failed: {err}");
                Err(err)
            }
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts, state: &S
    ) -> Result<Self, Self::Rejection> {
        // TODO: Make this token extraction a reusable fn for both this and previous extractor.
        let token = TypedHeader::<Authorization<Token>>::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                log::debug!("Failed to extract the token: {}", err);
                AppError::AuthUnauthorized
            })?;

        match token::verify_jwt(token.0 .0.token()) {
            Ok(claims) => Ok(claims),
            Err(err) => {
                log::debug!("Token verification failed: {err}");
                Err(err)
            }
        }
    }
}
