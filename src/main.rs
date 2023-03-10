use rocket::*;

mod api;
pub mod apiv2;
pub mod schema;
pub mod db;

#[options("/<_..>")]
fn all_options() {
    println!("Hit 'all_options' route");
    /* Intentionally left empty */
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![all_options])
        .mount("/receipts/api", api::routes())
        .mount("/apiv2", apiv2::routes())
        .attach(apiv2::fairing::CORS)
}
