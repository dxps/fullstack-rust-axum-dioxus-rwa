use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::{domain::model::UserId, AppError};

use super::validate_token_extract_claims;

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
