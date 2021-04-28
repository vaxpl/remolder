use rocket::http::{MediaType, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct Mime(pub MediaType);

impl Deref for Mime {
    type Target = MediaType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Mime {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let content_type = request.headers().get_one("content-type");
        match content_type {
            Some(content_type) => {
                if let Some(v) = MediaType::parse_flexible(content_type) {
                    Outcome::Success(Mime(v))
                } else {
                    Outcome::Failure((Status::BadRequest, ()))
                }
            }
            None => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}
