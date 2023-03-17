use diesel::prelude::*;
use rocket::serde::{ Deserialize, Serialize };
use crate::schema::user_friends;

#[derive(Debug, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserFriend {
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
    pub request_accepted: bool,
}


#[derive(Insertable)]
#[diesel(table_name = user_friends)]
pub struct NewUserFriend {
    pub user_id: i32,
    pub friend_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FriendRequestResponse<'a> {
    pub email: &'a str,
    pub reply: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FriendRequest<'a> {
    pub email: &'a str,
}