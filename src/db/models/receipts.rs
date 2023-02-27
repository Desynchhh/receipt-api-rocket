use diesel::prelude::*;
use chrono::NaiveDate;
use rocket::serde::Deserialize;
use crate::schema::receipts;

#[derive(Queryable)]
pub struct Receipt {
    pub id: i32,
    pub user_id: i32,
    pub store: String,
    pub date_bought: NaiveDate,
    pub subtotal: f32,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
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

#[derive(Insertable)]
#[diesel(table_name=receipts)]
pub struct PostReceipt<'a>{
    pub user_id: &'a i32,
    pub store: &'a str,
    pub date_bought: &'a str,
    pub subtotal: &'a f32,
}

#[derive(AsChangeset)]
#[diesel(table_name = receipts)]
pub struct UpdateReceipt {
    pub user_id: Option<i32>,
    pub store: Option<String>,
    pub date_bought: Option<NaiveDate>,
    pub subtotal: Option<f32>,
    pub updated_at: Option<NaiveDate>,
    pub is_deleted: Option<bool>,
}
