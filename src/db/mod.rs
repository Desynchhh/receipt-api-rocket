use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set! (.env)");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_|  panic!("Error connecting to db on address {}", database_url))
}
