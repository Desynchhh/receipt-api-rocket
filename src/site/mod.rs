use rocket::{ *,
    request::{ FromRequest, Outcome },
    http::{Status, CookieJar},
};
use rocket_dyn_templates::{ Template, tera::Context, };

mod receipts;
mod users;
mod test_routes;
pub mod utils;

pub const JWT_COOKIE_NAME: &str = "receipt_management_jwt";

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Template {
    let mut context = Context::new();

    // if let Some(_) = cookies.get_private(JWT_COOKIE_NAME) {
    //     context.insert("logged_in", &true);
    // }
    context.insert("logged_in", &true);

    context.insert("title", "Home");
    Template::render("index", context.into_json())
}
#[get("/temp")]
fn index_temp(jwt: JwtToken) -> Template {
    let mut context = Context::new();

    context.insert("title", "TEMP");
    Template::render("index", context.into_json())
}


pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![index, index_temp];
    routes.extend(receipts::routes());
    routes.extend(users::routes());
    routes.extend(test_routes::routes());
    routes
}


// Request guards
struct JwtToken {
    email: String,
    password: String,
    exp: i64,
}

impl JwtToken {
    fn from_jwt(jwt:users::utils::DecodedJwtUser) -> Self {
        Self {
            email: jwt.email,
            password: jwt.password,
            exp: jwt.exp
        }
    }
}

#[derive(Debug)]
enum JwtTokenError {
    Missing,
    Invalid,
    Expired,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = JwtTokenError;

    async fn from_request<'a>(request: &'r Request<'a>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private(JWT_COOKIE_NAME) {
            Some(cookie) => {
                println!("matched cookie!");
                let token = users::utils::decode_jwt_str(cookie.value());
                match token {
                    Ok(t) => Outcome::Success(JwtToken::from_jwt(t.claims)),
                    Err(_e) => Outcome::Failure((Status::Unauthorized, JwtTokenError::Invalid))
                }
                
            },
            None => {
                println!("did not match cookie...");
                match request.headers().get_one("Authorization") {
                    Some(header) => {
                        println!("matched header!");
                        let token = header.split(' ').last().unwrap();
                        let token = users::utils::decode_jwt_str(token);
                        match token {
                            Ok(t) => Outcome::Success(JwtToken::from_jwt(t.claims)),
                            Err(_e) => Outcome::Failure((Status::Unauthorized, JwtTokenError::Invalid))
                        }
                    },
                    None => {
                        println!("did not match header...");
                        Outcome::Failure((Status::Unauthorized, JwtTokenError::Missing))
                    } 
                }
            } 
        }
    }
}
