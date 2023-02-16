use rocket::{ *, fs::{ FileServer, relative} };
use rocket_dyn_templates::Template;

mod site;
mod api;
pub mod apiv2;
pub mod schema;
pub mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/receipts/api", api::routes())
        .mount("/", site::routes())
        .mount("/public", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
