use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use crate::schema::users;

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub api_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
}