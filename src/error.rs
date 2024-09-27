use axum::{http::StatusCode, response::{IntoResponse, Response}};
use log::{error, warn};
use serde_json::json;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFoundError,
    CanNotLockState,
    DBSelectError,
    DBInsertError,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		warn!("->> {:<12} - {self:?}", "ERROR OCCURRED");
        let msg = json!({
            "status": "error",
            "error_code": 500,
        });

        (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()).into_response()
	}
}