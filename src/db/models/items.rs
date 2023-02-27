use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::items;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub receipt_id: i32,
    pub price: f32,
    pub discount: Option<f32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub receipt_id: &'a i32,
    pub price: &'a f32,
    pub discount: Option<&'a f32>,
}
