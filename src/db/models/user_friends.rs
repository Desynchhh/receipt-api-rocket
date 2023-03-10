use diesel::prelude::*;
use crate::schema::user_friends;

#[derive(Queryable)]
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
