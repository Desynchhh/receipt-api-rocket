use crate::apiv2::request_guard::JwtToken;
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde::Serialize;

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

#[derive(Queryable, Serialize, Debug)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
#[diesel(table_name = users)]
pub struct UserDetails {
    pub id: i32,
    email: String,
    first_name: String,
    last_name: String,
}

impl UserDetails {
	pub fn new(id: i32, email: String, first_name: String, last_name: String) -> Self {
		Self {
			id,
			email,
			first_name,
			last_name,
		}
	}

	pub fn from_jwt(jwt: JwtToken) -> Self {
		Self {
			id: jwt.id,
			email: jwt.email,
			first_name: jwt.first_name,
			last_name: jwt.last_name,
		}
	}
}
