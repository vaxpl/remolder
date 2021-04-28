use rocket::http::{ContentType, Status};
use rocket::response::{self, Responder};
use rocket::{Request, Response};
use serde::Serialize;
use std::io::Cursor;

/// The message for api failure.
#[derive(Debug, Serialize)]
pub struct ApiFailure<E> {
    pub errors: Option<E>,
    pub status: Option<&'static str>,
}

impl<E> ApiFailure<E> {
    /// Construct a new api failure message.
    pub fn new(errors: Option<E>) -> Self {
        Self {
            errors,
            status: Some("failure"),
        }
    }
}

impl<U> From<U> for ApiFailure<String>
where
    // U: std::error::Error,
    U: std::string::ToString,
{
    fn from(val: U) -> Self {
        Self {
            errors: Some(val.to_string()),
            status: Some("failure"),
        }
    }
}

impl<'r, E> Responder<'r> for ApiFailure<E>
where
    E: Serialize,
{
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let (status, body) = serde_json::to_string(&self).map_or_else(
            |e| (Status::InternalServerError, e.to_string()),
            |v| (Status::BadRequest, v),
        );

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(Cursor::new(body))
            .ok()
    }
}

/// The message for api success.
#[derive(Debug, Serialize)]
pub struct ApiSuccess<D, M> {
    pub data: Option<D>,
    pub meta: Option<M>,
    pub status: Option<&'static str>,
}

impl<D, M> ApiSuccess<D, M> {
    /// Construct a new api success message.
    pub fn new(data: Option<D>, meta: Option<M>) -> Self {
        Self {
            data,
            meta,
            status: Some("success"),
        }
    }
}

impl<'r, D, M> Responder<'r> for ApiSuccess<D, M>
where
    D: Serialize,
    M: Serialize,
{
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let (status, body) = serde_json::to_string(&self).map_or_else(
            |e| (Status::InternalServerError, e.to_string()),
            |v| (Status::Ok, v),
        );

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(Cursor::new(body))
            .ok()
    }
}

/// The message for api response.
#[derive(Debug, Serialize)]
pub enum ApiResponse<D, E, M> {
    Failure(ApiFailure<E>),
    Success(ApiSuccess<D, M>),
}

impl<'r, D, E, M> Responder<'r> for ApiResponse<D, E, M>
where
    D: Serialize,
    E: Serialize,
    M: Serialize,
{
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let (status, body) = match self {
            ApiResponse::Failure(v) => serde_json::to_string(&v).map_or_else(
                |e| (Status::InternalServerError, e.to_string()),
                |v| (Status::BadRequest, v),
            ),
            ApiResponse::Success(v) => serde_json::to_string(&v).map_or_else(
                |e| (Status::InternalServerError, e.to_string()),
                |v| (Status::Ok, v),
            ),
        };

        Response::build()
            .header(ContentType::JSON)
            .status(status)
            .sized_body(Cursor::new(body))
            .ok()
    }
}

/// Type alias for the Result of a api call.
pub type ApiResult<D, E, M> = Result<ApiSuccess<D, M>, ApiFailure<E>>;
