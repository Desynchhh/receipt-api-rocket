use rocket::{ *,
  form::{ FromForm, Form },
  http::CookieJar,
  serde::{json::Json, Serialize}
}; 
use crate::{apiv2, db::models::users::User};

pub mod utils;

#[derive(FromForm)]
pub struct UserRegisterForm<'r> {
  #[field(name = "firstName")]
  first_name: &'r str,
  
  #[field(name = "lastName")]
  last_name: &'r str,

  #[field(name = "email")]
  email: &'r str,

  #[field(name = "password")]
  password: &'r str,
  
  #[field(name = "passwordConfirm")]
  confirm_password: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
enum HttpPostResponse<S, F> {
  Success(S),
  Failure(F)
}

#[derive(FromForm)]
pub struct UserLoginForm<'r> {
  email: &'r str,
  password: &'r str
}

#[post("/users/create", data = "<form>")]
async fn create(form: Form<UserRegisterForm<'_>>,) -> Json<HttpPostResponse<User, Vec<String>>> {
  let errors = utils::validate_form_input(&form);
  if errors.len() > 0 {
      let res = HttpPostResponse::Failure(errors);
      return Json::from(res);
  }

  let password = utils::encrypt_password(form.password);
  let new_user = utils::create_user_object(&form, &password);
  let user = apiv2::methods::create_user(new_user);
  let res = HttpPostResponse::Success(user);

  Json::from(res)
}

#[post("/users/login", data = "<form>")]
fn login(form: Form<UserLoginForm<'_>>, cookies: &CookieJar<'_>) -> Json<HttpPostResponse<String, Vec<String>>> {
  fn error_response() -> HttpPostResponse<String, Vec<String>> {
    let error = vec!["Incorrect email or password.".to_string()];
    let res = HttpPostResponse::Failure(error);
    return res;
  }

  use apiv2::methods::{ get_user, GetByField };
  let user_email = GetByField::Email(form.email.to_string());
  let user = get_user(user_email);
  if let Err(err) = &user {
    println!("{:?}", err);
    return Json::from(error_response());
  }

  let user = user.unwrap();
  if !utils::verify_password(form.password, &user) {
      return Json::from(error_response());
  }

  let jwt = utils::build_jwt_cookie(&user);
  cookies.add_private(jwt.clone());
  let res = HttpPostResponse::Success(jwt.value().to_string());
  Json::from(res)
}

#[post("/users/logout")]
fn logout(cookies: &CookieJar<'_>) -> Json<HttpPostResponse<bool, bool>> {
  utils::remove_jwt_cookie(cookies);
  Json::from(HttpPostResponse::Success(true))
}

pub fn routes() -> Vec<rocket::Route> {
  routes![
      create,
      login,
      logout,
  ]
}