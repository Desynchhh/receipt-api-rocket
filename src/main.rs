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

// fn main() {
//     create_test_users();
// }


fn create_test_users() {
    use diesel::prelude::*;
    use crate::schema::users;
    use db::models::users as user_models;
    
    let connection = &mut db::establish_connection();

    let new_user = user_models::NewUser {
        email: "testmail123@gmail.com",
        first_name: "testFirst",
        last_name: "testLast",
        password: "P@55w0rd!"
    };
    
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving test user.");
}