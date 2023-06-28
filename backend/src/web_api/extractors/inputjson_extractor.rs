use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, MatchedPath},
    http::Request,
    http::StatusCode,
    Json, RequestPartsExt,
};
use serde_json::{json, Value};
use std::error::Error;

/// This extractor provides an expected JSON based API response
/// with the error that `axum::Json` could potentially return.
pub struct InputJson<T>(pub T);

// ----------------------------------------------------------------------------
// Manual implementation of `FromRequest` that wraps `axum::Json` extractor.
// ----------------------------------------------------------------------------
// Pros&Cons:
// + Powerful API: Implementing `FromRequest` grants access to `RequestParts`
//   and `async/await`. This means that you can create more powerful rejections
// - Boilerplate: Requires creating a new extractor for every custom rejection
// - Complexity: Manually implementing `FromRequest` results on more complex code
#[async_trait]
impl<S, B, T> FromRequest<S, B> for InputJson<T>
where
    Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = (StatusCode, Json<Value>);

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        //
        let (mut parts, body) = req.into_parts();

        // We can use other extractors to provide better rejection messages.
        // For example, here we are using `axum::extract::MatchedPath` to
        // provide a better error message.

        // This must run first since `Json` extraction consumes the request.
        let path = parts
            .extract::<MatchedPath>()
            .await
            .map(|path| path.as_str().to_owned())
            .ok();

        let req = Request::from_parts(parts, body);

        match Json::<T>::from_request(req, state).await {
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
                    Json(json!({
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
    find_error_source::<serde_json::Error>(err).map(|serde_json_err| serde_json_err.to_string())
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
