use rocket::*;

mod api;


#[launch]
fn rocket() -> _ {
    rocket::build()
        // .mount("/file", routes![files])
        .mount("/receipts/api", api::routes())
}