use crate::{
    web_api::token::{verify_jwt, Claims, Token},
    AppError,
};
use axum::{extract::FromRequestParts, headers::Authorization, http::request::Parts, TypedHeader};

/// It extracts the token - if it exists - from the Authorization (HTTP request) header having the value of "Token <token>".
pub async fn validate_token_extract_claims<S: Send + Sync>(
    parts: &mut Parts,
    state: &S,
) -> Result<Claims, AppError> {
    //
    let token = TypedHeader::<Authorization<Token>>::from_request_parts(parts, state)
        .await
        .map_err(|err| {
            log::debug!("Failed to extract the token: {}", err);
            AppError::Unauthorized(err.to_string())
        })?;

    match verify_jwt(token.0 .0.token()) {
        Ok(claims) => Ok(claims),
        Err(err) => {
            log::debug!("Failed to verify jwt: {err}");
            Err(err)
        }
    }
}
