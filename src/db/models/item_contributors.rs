use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::item_contributors;

#[derive(Queryable)]
pub struct ItemContributor {
    pub id: i32,
    pub user_id: i32,
    pub item_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = item_contributors)]
pub struct NewItemContributor<'a> {
    pub user_id: &'a i32,
    pub item_id: &'a i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = item_contributors)]
pub struct UpdateItemContributor {
    pub user_id: Option<i32>,
    pub item_id: Option<i32>,
    pub is_deleted: Option<bool>
}