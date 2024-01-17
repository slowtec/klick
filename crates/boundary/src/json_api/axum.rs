use super::Error;
use serde::Serialize;

impl<T> axum::response::IntoResponse for Error<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.status, axum::response::Json(self)).into_response()
    }
}
