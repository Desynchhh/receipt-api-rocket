use diesel::prelude::*;
use chrono::NaiveDate;
use crate::schema::items;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub receipt_id: i32,
    pub price: f32,
    pub discount: Option<f32>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub receipt_id: i32,
    pub price: f32,
    pub discount: Option<f32>,
}
