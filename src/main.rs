use rocket::*;

mod api;
pub mod apiv2;
pub mod schema;
pub mod db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/receipts/api", api::routes())
        .mount("/apiv2", apiv2::routes())
        .attach(apiv2::fairing::CORS)
}
