use rocket::{ *,
    form::{ FromForm, Form },
    response::{ Redirect, Flash },
    http::CookieJar,
    request::FlashMessage
}; 
use rocket_dyn_templates::{
    Template,
    tera::Context
};
use crate::apiv2;

pub mod utils;

#[derive(FromForm)]
pub struct UserRegisterForm<'r> {
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

#[derive(FromForm)]
pub struct UserLoginForm<'r> {
    email: &'r str,
    password: &'r str
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
async fn create_user_post(form: Form<UserRegisterForm<'_>>,) -> Flash<Redirect> {
    let errors = utils::validate_form_input(&form);
    if errors.len() > 0 {
        let mut error_string = String::new();
        for error in errors {
            error_string.push_str(format!("{}", error).as_str());
        }
        return Flash::error(Redirect::to("/users/create"), error_string);
    }

    let password = utils::encrypt_password(form.password);
    let new_user = utils::create_user_object(&form, &password);
    let _ = apiv2::methods::create_new_user(new_user);

    Flash::success(Redirect::to("/users/login"), "User created successfully! You can now log in.")
}

#[get("/users/login")]
fn login_screen(flash: Option<FlashMessage<'_>>) -> Template {
    let mut context = Context::new();
    if let Some(flash_message) = flash.map(|msg| format!("{}", msg.message())) {
        context.insert("flash_message", &flash_message)
    }

    Template::render("users/login", context.into_json())
}

#[post("/users/login", data = "<form>")]
fn user_login(form: Form<UserLoginForm<'_>>, cookies: &CookieJar<'_>) -> Result<Redirect, Flash<Redirect>> {
    let user = apiv2::methods::get_user(form.email.to_string());
    if let Err(err) = &user {
        return Err(Flash::error(Redirect::to("/users/login"), format!("Incorrect email or password. --- {}", err)));
    }

    let user = user.unwrap();
    if !utils::verify_password(form.password, &user) {
        return Err(Flash::error(Redirect::to("/users/login"), "Incorrect email or password."));
    }

    let jwt = utils::build_jwt_cookie(&user);
    cookies.add_private(jwt);
    Ok(Redirect::to(uri!(super::index)))
}

#[get("/users/logout")]
fn user_logout(cookies: &CookieJar<'_>) -> Redirect {
    utils::remove_jwt_cookie(cookies);
    Redirect::to(uri!("/"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_user_get,
        create_user_post,
        login_screen,
        user_login,
        user_logout,
    ]
}