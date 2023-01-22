use std::error::Error;

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

// ----------------------------------------------------------------------------
// Manual implementation of `FromRequest` that wraps `axum::Json` extractor.
// ----------------------------------------------------------------------------
// Pros&Cons:
// + Powerful API: Implementing `FromRequest` grants access to `RequestParts`
//   and `async/await`. This means that you can create more powerful rejections
// - Boilerplate: Requires creating a new extractor for every custom rejection
// - Complexity: Manually implementing `FromRequest` results on more complex code
use axum::{
    extract::{rejection::JsonRejection, FromRequest, MatchedPath},
    http::Request,
    http::StatusCode,
    RequestPartsExt,
};
use serde_json::{json, Value};

// This is our `InputJson` extractor that customizes the error from `axum::Json`.
pub struct InputJson<T>(pub T);

#[async_trait]
impl<S, B, T> FromRequest<S, B> for InputJson<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();
        log::debug!("[FromRequest for Json]");

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.
        //
        // Have to run that first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // Convert the error from `axum::Json` into whatever we want.
            Err(rejection) => {
                let message = rejection.to_string();
                let reason = find_serde_json_error_source(&rejection);

                let code = match rejection {
                    JsonRejection::JsonDataError(_) => StatusCode::UNPROCESSABLE_ENTITY,
                    JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
                    JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                Err((
                    code,
                    axum::Json(json!({
                        "error": message,
                        "path": path,
                        "reason": reason,
                    })),
                ))
            }
        }
    }
}

// It attempts to extract the inner `serde_json::Error`, if that succeeds we can
// provide a more specific error
fn find_serde_json_error_source<E>(err: &E) -> Option<String>
where
    E: Error + 'static,
{
    if let Some(serde_json_err) = find_error_source::<serde_json::Error>(err) {
        Some(serde_json_err.to_string())
    } else {
        None
    }
}

// It attempts to downcast `err` into a `T`, and if that fails,
// recursively try and downcast `err`'s source.
fn find_error_source<'a, T>(err: &'a (dyn Error + 'static)) -> Option<&'a T>
where
    T: Error + 'static,
{
    if let Some(err) = err.downcast_ref::<T>() {
        Some(err)
    } else if let Some(source) = err.source() {
        find_error_source(source)
    } else {
        None
    }
}
