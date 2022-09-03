use axum::{
    async_trait,
    extract::{FromRequest, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use crate::{domain::model::UserId, jwt, AppError};

#[async_trait]
impl<B> FromRequest<B> for UserId
where
    B: Send,
{
    type Rejection = AppError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the Authorization HTTP header with "Bearer <token>" value.
        let token = TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|err| {
                log::debug!("Failed to extract the bearer token: {}", err);
                AppError::AuthUnauthorizedErr
            })?;
        match jwt::verify(token.token()) {
            Ok(claims) => {
                // We just extract and provide the user (aka jwt subject) id.
                Ok(claims.sub.into())
            }
            Err(err) => {
                log::debug!("Token verification failed: {err}");
                Err(AppError::InvalidTokenErr)
            }
        }
    }
}
