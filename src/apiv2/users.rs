use crate::{
  apiv2::{self, HttpPostResponse},
  db::models::{
    user_friends::{NewUserFriend, FriendRequestResponse, FriendRequest},
    users::{ User, UserDetails }
  },
};
use rocket::{
  *,
  form::{Form, FromForm},
  http::CookieJar,
  serde::json::Json,
};

use super::{
  methods::{create_friend, get_user, GetByField},
  request_guard::JwtToken,
};

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

#[derive(FromForm)]
pub struct UserLoginForm<'r> {
  email: &'r str,
  password: &'r str,
}

#[post("/users/create", data = "<form>")]
async fn create(form: Form<UserRegisterForm<'_>>) -> Json<HttpPostResponse<User, Vec<String>>> {
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
fn login(
  form: Form<UserLoginForm<'_>>,
  cookies: &CookieJar<'_>,
) -> Json<HttpPostResponse<String, Vec<String>>> {
  fn error_response() -> HttpPostResponse<String, Vec<String>> {
    let error = vec!["Incorrect email or password.".to_string()];
    let res = HttpPostResponse::Failure(error);
    return res;
  }
  
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

#[post("/users/add-friend", data = "<friend_info>")]
fn add_friend(
  jwt: JwtToken,
  friend_info: Json<FriendRequest>,
) -> Json<HttpPostResponse<String, String>> {
  let friend_info = friend_info.into_inner();
  println!("{:?}", &friend_info);
  let friend = get_user(GetByField::Email(
    friend_info.email.to_string(),
  ));
  if friend.is_err() {
    return Json::from(HttpPostResponse::Failure(format!(
      "Could not find a user with the email \"{}\"",
      "EMAIL-PROVIDED"
    )));
  }
  
  let friend = friend.unwrap();
  
  let new_user_friend = NewUserFriend {
    user_id: jwt.id,
    friend_id: friend.id,
  };
  
  let new_friends = create_friend(new_user_friend);
  
  if new_friends.is_err() {
    return Json::from(HttpPostResponse::Failure(
      "Something went wrong during friend creation.".to_string(),
    ));
  }
  
  Json::from(HttpPostResponse::Success(
    "Friend request sent!".to_string(),
  ))
}

#[post("/users/friend-request-response", data = "<response>")]
fn accept_friend_request(jwt: JwtToken, response: Json<FriendRequestResponse>) -> Json<HttpPostResponse<String, String>> {
  let response = response.into_inner();
  let friend = get_user(GetByField::Email(response.email.to_string()));
  if friend.is_err() {
    // TODO: Delete the friend request if the friend doesn't exist (this should theoretically never happen, unless a user has been deleted)
    // apiv2::methods::delete_friend(&jwt.id, &friend_id);
    return Json::from(HttpPostResponse::Failure("User not found.".to_string()));
  }
  let friend = friend.unwrap();
  
  match response.reply {
    true => {
      let accept_is_successful = apiv2::methods::accept_friend_request(&jwt.id, &friend.id);
      if accept_is_successful.is_err() {
        return Json::from(HttpPostResponse::Failure("Could not accept friend request.".to_string()));
      }
      
      return Json::from(HttpPostResponse::Success("Friend request accepted!".to_string()));
    },
    false => {
      let deny_is_successful = apiv2::methods::delete_friend(&jwt.id, &friend.id);
      if !deny_is_successful {
        return Json::from(HttpPostResponse::Failure("Could not delete friend request.".to_string()));
      }
      
      return Json::from(HttpPostResponse::Success("Friend request denied!".to_string()));
    },
  }
}

#[get("/users/friends")]
fn get_friends(jwt: JwtToken) -> Json<Vec<UserDetails>> {
  let friends = apiv2::methods::get_friends(&jwt.id);
  if friends.is_err() {
    return Json::from(Vec::new());
  }
  let mut friends = friends.unwrap();
  let logged_in_user = UserDetails::from_jwt(jwt);
  
  friends.push(logged_in_user);
  
  Json::from(friends)
}

#[get("/users/pending-requests")]
fn get_pending_requests(jwt: JwtToken) -> Json<Vec<UserDetails>> {
  let friend_requests = apiv2::methods::get_pending_friend_requests(&jwt.id);
  
  if friend_requests.is_err() {
    return Json::from(Vec::new());
  }
  
  Json::from(friend_requests.unwrap())
}

pub fn routes() -> Vec<rocket::Route> {
  routes![
  create,
  login,
  logout,
  add_friend,
  accept_friend_request,
  get_pending_requests,
  get_friends,
  ]
}
