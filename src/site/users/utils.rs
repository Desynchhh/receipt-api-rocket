use bcrypt;
use regex::Regex;
use rocket::http::Cookie;
use rocket::serde::{Serialize, Deserialize};
use rocket::time::{OffsetDateTime, Duration};
use crate::apiv2;
use crate::db::models::users::{NewUser, User};
use super::UserRegisterForm;
use super::super::JWT_COOKIE_NAME;
use rocket::form::Form;
use jsonwebtoken::{ encode, decode, Header, EncodingKey };
use std::env;
use dotenvy::dotenv;

const EMAIL_REGEX:&str = r"^[a-zA-Z0-9]+@[a-zA-Z0-9]+\..+$";


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct JwtUser<'u> {
    email: &'u str,
    password: &'u str,
}

impl<'u> JwtUser<'u> {
    fn from_user(user: &'u User) -> Self {
        Self {
            email: &user.email,
            password: &user.password
        }
    }
}


pub fn validate_form_input(form: &Form<UserRegisterForm>) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    
    if !form.password.eq(form.confirm_password) {
        errors.append(&mut vec!["Passwords do not match.".to_string()]);
    }

    let re = Regex::new(EMAIL_REGEX).unwrap();
    if !re.is_match(form.email) {
        errors.append(&mut vec!["Please enter a valid email address.".to_string()]);
    }

    let registered_emails = apiv2::methods::get_all_user_emails();
    if registered_emails.contains(&form.email.to_owned()) {
        errors.append(&mut vec!["An account with that email already exists.".to_string()]);
    }

    if form.first_name.len() < 1 {
        errors.append(&mut vec!["You must enter your first name to create an account.".to_string()]);
    }

    if form.last_name.len() < 1 {
        errors.append(&mut vec!["You must enter your last name to create an account.".to_string()]);
    }

    errors
}

pub fn create_user_object<'a>(form: &'a Form<UserRegisterForm<'_>>, password:&'a str) -> NewUser<'a> {
    NewUser {
        email: form.email,
        first_name: form.first_name,
        last_name: form.last_name,
        password
    }
}

pub fn encrypt_password(password: &str) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, user: &User) -> bool {
    bcrypt::verify(password, &user.password).unwrap()
}

pub fn build_jwt_cookie(user: &User) -> Cookie<'static> {
    let jwt = create_jwt(user);
    let expiration = OffsetDateTime::now_utc() + Duration::days(7);

    Cookie::build(JWT_COOKIE_NAME, jwt)
        .secure(true)
        .http_only(true)
        .expires(expiration)
        .finish()
}

fn create_jwt(user: &User) -> String {
    dotenv().ok();
    let user = JwtUser::from_user(user);
    let secret = env::var("JWT_TOKEN_SECRET").expect("JWT_TOKEN_SECRET not set.");
    let jwt = encode(
    &Header::default(),
    &user,
    &EncodingKey::from_secret(secret.as_ref())
    ).expect("Error in create_jwt");
    println!("{}", jwt);
    String::from("INSERT JWT HERE")
}