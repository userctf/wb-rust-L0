use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorType {
    NotFoundError,
}

#[derive(Debug)]
pub struct Error {
    error_type: ErrorType,
}

impl Error {
    pub fn new (error_type: ErrorType) -> Self {
        Error {error_type}
    }

    pub fn new_not_found() -> Self {
        Error::new(ErrorType::NotFoundError)
    }
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - {self:?}", "ERROR OCCURRED");
        let msg = json!({
            "error_code": 500,
        });

        (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()).into_response()
	}
}