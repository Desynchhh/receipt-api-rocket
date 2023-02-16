use rocket::{
    *,
    http::Status,
    request::{ Outcome, FromRequest },
};
use super::JWT_COOKIE_NAME;

struct JwtCookie {
    value: String
}

#[derive(Debug)]
enum JwtCookieError {
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtCookie {
    type Error = JwtCookieError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        println!("headers: {:#?}", req.headers());
        match req.cookies().get_private(JWT_COOKIE_NAME) {
            None => Outcome::Failure((Status::Unauthorized, JwtCookieError::Missing)),
            Some(c) => {
                let cookie = JwtCookie {
                    value: c.value().to_string(),
                };
                Outcome::Success(cookie)
            },
        }
    }
}