use rocket::{
  Request,
  http::Status,
  request::{ FromRequest, Outcome },
};
use crate::apiv2::{
  JWT_COOKIE_NAME,
  users::utils::{
    DecodedJwtUser,
    decode_jwt_str,
  }
};

// Request guards
#[derive(Debug)]
pub struct JwtToken {
  pub id: i32,
  pub email: String,
  // password: String,
  // exp: i64,
}

impl JwtToken {
  fn from_jwt(jwt:DecodedJwtUser) -> Self {
    Self {
        id: jwt.id,
        email: jwt.email,
        // password: jwt.password,
        // exp: jwt.exp
    }
  }
}

#[derive(Debug)]
pub enum JwtTokenError {
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
              let token = decode_jwt_str(cookie.value());
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
                      let token = decode_jwt_str(token);
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