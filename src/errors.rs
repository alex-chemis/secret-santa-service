use thiserror::Error;
use std::io::Cursor;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Result, Response, Responder};
use rocket::serde::{Serialize, json::serde_json };

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    message: String,
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    BadRequest(String),
}

impl Error {
    fn get_http_status(&self) -> Status {
        match self {
            Error::Internal(_) => Status::InternalServerError,
            Error::NotFound(_) => Status::NotFound,
            Error::Forbidden(_) => Status::Forbidden,
            _ => Status::BadRequest,
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        // serialize struct into json string
        let err_response = serde_json::to_string(&ErrorResponse{
            message: self.to_string()
        }).unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}
