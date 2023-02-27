use rocket::{
  *, Request, Response,
  http::{ Status, Header },
  fairing::{ Fairing, Info, Kind },
  request::{ FromRequest, Outcome },
};

pub mod methods;
pub mod users;

pub const JWT_COOKIE_NAME: &str = "receipt_management_jwt";

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  routes.extend(users::routes());
  routes
}


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // response.set_header(Header::new("Access-Control-Allow-Origin", "http://127.0.0.1:5173"));
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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