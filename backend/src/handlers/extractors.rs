use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::Authorization,
    http::request::Parts,
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

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = validate_token_extract_claims(parts, state).await?;
        Ok(claims.sub.into())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        validate_token_extract_claims(parts, state).await
    }
}

// It extracts the token - if it exists - from the Authorization (HTTP request) header having the value of "Bearer <token>".
async fn validate_token_extract_claims<S: Send + Sync>(
    parts: &mut Parts,
    state: &S,
) -> Result<Claims, AppError> {
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
