use rocket::{ *, form::{ FromForm, Form }, response::{Redirect, Flash}, request::FlashMessage };
use rocket_dyn_templates::{
    Template,
    tera::Context
};
use bcrypt;
use regex::Regex;
use crate::apiv2;
use crate::db::models::users::NewUser;

const EMAIL_REGEX:&str = r"^[a-zA-Z0-9]+@[a-zA-Z0-9]+\..+$";


#[derive(FromForm)]
struct UserForm<'r> {
    #[field(name = "first-name")]
    first_name: &'r str,
    #[field(name = "last-name")]
    last_name: &'r str,
    #[field(name = "email")]
    email: &'r str,
    #[field(name = "password")]
    password: &'r str,
    #[field(name = "password-confirm")]
    confirm_password: &'r str
}

#[get("/users/create")]
async fn create_user_get(flash: Option<FlashMessage<'_>>) -> Template {
    let mut context = Context::new();
    if let Some(flash_message) = flash.map(|msg| format!("{}", msg.message())) {
        context.insert("flash_message", &flash_message)
    }
    Template::render("users/create_user", context.into_json())
}

#[post("/users/create", data = "<form>")]
async fn create_user_post(form: Form<UserForm<'_>>,) -> Flash<Redirect> {
    let mut errors: Vec<String> = Vec::new();
    let re = Regex::new(EMAIL_REGEX).unwrap();

    if !form.password.eq(form.confirm_password) {
        errors.append(&mut vec!["Passwords do not match.".to_string()]);
    }
    if !re.is_match(form.email) {
        errors.append(&mut vec!["Please enter a valid email address.".to_string()]);
    }
    let registered_emails = apiv2::methods::get_all_user_emails();
    if registered_emails.contains(&form.email.to_owned()) {
        errors.append(&mut vec!["An account with that email already exists.".to_string()]);
        // errors.insert("email_exists".to_string(), ());
    }
    if form.first_name.len() < 1 {
        errors.append(&mut vec!["You must enter your first name to create an account.".to_string()]);
        // errors.insert("first_name".to_string(), ());
    }
    if form.last_name.len() < 1 {
        errors.append(&mut vec!["You must enter your last name to create an account.".to_string()]);
        // errors.insert("last_name".to_string(), ());
    }

    let has_errors = errors.len() > 0;

    let mut error_string = String::new();
    if has_errors {
        for error in errors {
            error_string.push_str(format!("{}", error).as_str());
        }
        return Flash::error(Redirect::to("/users/create"), error_string);
    }

    let hashed_password = bcrypt::hash(form.password, bcrypt::DEFAULT_COST).unwrap();
    let new_user = NewUser {
        email: form.email,
        first_name: form.first_name,
        last_name: form.last_name,
        password: &hashed_password
    };
    let _ = apiv2::methods::create_new_user(new_user);

    Flash::success(Redirect::to("/users/login"), "User created successfully!")
}

#[get("/users/login")]
fn login_screen() -> Template {
    Template::render("users/login", Context::new().into_json())
}


#[post("/users/login")]
fn user_login() -> Redirect {
    Redirect::to(uri!(super::index))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_user_get,
        create_user_post,
        login_screen,
        user_login,
    ]
}