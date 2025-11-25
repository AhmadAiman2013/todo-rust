use axum::extract::{FromRequest, FromRequestParts, Request, Path as AxumPath, rejection::JsonRejection::*};
use axum::http::request::Parts;
use axum::Json;
use log::log;
use serde::de::DeserializeOwned;
use crate::error::AppError;

pub struct SafeJson<T>(pub T);

impl<S, T> FromRequest<S> for SafeJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(SafeJson(value)),
            Err(err) => {
                log::warn!("JSON extraction error: {:?}", err);
                let app_err = match err {
                    JsonDataError(_) => AppError::BadRequest("Invalid JSON data".into()),
                    JsonSyntaxError(_) => AppError::BadRequest("Malformed JSON syntax".into()),
                    MissingJsonContentType(_) => AppError::BadRequest("Missing Content-Type header".into()),
                    BytesRejection(_) => AppError::BadRequest("Unable to process request".into()),
                    _ => AppError::BadRequest("Invalid JSON request".into()),
                };

                Err(app_err)
            }
        }
    }
}

pub struct SafePath<T>(pub T);

impl<S, T> FromRequestParts<S> for SafePath<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AxumPath::<T>::from_request_parts(parts, state).await {
            Ok(p) => Ok(SafePath(p.0)),
            Err(e) => {
                log::warn!("Path extraction error: {:?}", e);
                Err(AppError::BadRequest("Invalid path parameter".into()))
            }
        }
    }
}