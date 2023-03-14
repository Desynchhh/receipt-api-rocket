use rocket::{
  *,
  serde::Serialize
};

pub mod methods;
pub mod users;
pub mod receipts;
pub mod request_guard;
pub mod fairing;

pub const JWT_COOKIE_NAME: &str = "receipt_management_jwt";


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum HttpPostResponse<S, F> {
  Success(S),
  Failure(F)
}

pub fn routes() -> Vec<rocket::Route> {
  let mut routes = routes![];
  routes.extend(users::routes());
  routes.extend(receipts::routes());
  routes
}
