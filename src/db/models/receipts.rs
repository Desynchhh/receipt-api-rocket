use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
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

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=receipts)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct NewReceipt {
    pub user_id: i32,
    pub store: String,
    pub date_bought: String,
    pub subtotal: f32,
}

#[derive(AsChangeset)]
#[diesel(table_name = receipts)]
pub struct UpdateReceipt {
    pub user_id: Option<i32>,
    pub store: Option<String>,
    pub date_bought: Option<NaiveDateTime>,
    pub subtotal: Option<f32>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: Option<bool>,
}
