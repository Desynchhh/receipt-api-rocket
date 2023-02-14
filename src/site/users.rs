use rocket::{
    *,
    form::{
        FromForm,
        Form
    }
};
use rocket_dyn_templates::{
    Template,
    tera::Context
};
use crate::apiv2;
use crate::db::models::users::NewUser;


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
async fn create_user_get() -> Template {
    let mut context = Context::new();
    Template::render("users/create_user", context.into_json())
}

#[post("/users/create", data = "<form>")]
async fn create_user_post(form: Form<UserForm<'_>>) -> Template {
    let new_user = NewUser {
        email: form.email,
        first_name: form.first_name,
        last_name: form.last_name,
        password: form.password
    };
    let created_user = apiv2::create_new_user(new_user);
    let mut context:Context = Context::new();
    context.insert("user", &created_user);
    Template::render("users/user_created", context.into_json())
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_user_get,
        create_user_post
    ]
}