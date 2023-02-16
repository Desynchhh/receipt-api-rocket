use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::receipts;

#[derive(Queryable)]
pub struct Receipt {
    pub id: i32,
    pub user_id: i32,
    pub store: String,
    pub date_bought: NaiveDateTime,
    pub subtotal: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_deleted: bool,
}

#[derive(Insertable)]
#[diesel(table_name=receipts)]
pub struct NewReceipt {
    pub user_id: i32,
    pub store: String,
    pub date_bought: String,
    pub subtotal: f32,
}