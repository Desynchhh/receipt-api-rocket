use rocket::*;
use rocket_dyn_templates::{ Template, tera::Context };

mod receipts;
mod users;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("title", "Home");
    Template::render("index", context.into_json())
}

pub fn routes() -> Vec<rocket::Route> {
    let mut routes = routes![index];
    routes.extend(receipts::routes());
    routes.extend(users::routes());
    routes
}
