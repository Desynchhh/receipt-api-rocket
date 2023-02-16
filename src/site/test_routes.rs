use rocket::{
    *,
    http::{ Cookie, CookieJar, Status },
    request::{Outcome, FromRequest},
    // time::{Duration, OffsetDateTime}
};

struct JwtCookie {
    // name: String,
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
        match req.cookies().get_private("key") {
            None => Outcome::Failure((Status::Unauthorized, JwtCookieError::Missing)),
            Some(c) => {
                let cookie = JwtCookie {
                    // name: c.name().to_string(),
                    value: c.value().to_string(),
                };
                Outcome::Success(cookie)
            },
            // Some(_) => Outcome::Failure((Status::BadRequest, CookieValueError::Invalid)),
        }
    }
}

#[get("/cookie/set")]
fn set_cookie(cookies: &CookieJar<'_>) -> &'static str {
    // let now = OffsetDateTime::now_utc();
    let cookie = Cookie::build("key", "here should be a jwt instead")
        .http_only(true)
        .secure(true)
        // .expires(now + Duration::minutes(1))
        .finish();
    cookies.add_private(cookie);
    println!();
    "cookie has been set."
}

#[get("/cookie/get")]
fn get_cookie(cookie: JwtCookie) -> String {
    println!("cookie value: {}", cookie.value);
    cookie.value
}

#[get("/cookie/remove")]
fn remove_cookie(cookies: &CookieJar<'_>) -> &'static str {
    cookies.remove_private(Cookie::named("key"));
    "cookie deleted."
}


pub fn routes() -> Vec<rocket::Route> {
    routes![set_cookie, get_cookie, remove_cookie]
}