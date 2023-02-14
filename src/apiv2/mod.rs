use diesel::prelude::*;
use crate::schema::{users, user_friends};
use crate::db::{
    establish_connection,
    models::users::{User, NewUser}
};

pub fn create_new_user(new_user: NewUser) -> User {
    let connection = &mut establish_connection();

    let _ = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user to db");

    users::table
        .filter(users::email.eq(new_user.email))
        .first(connection)
        .expect("Error loading user after creation")
}
