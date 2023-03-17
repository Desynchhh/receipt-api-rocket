use diesel::prelude::*;
use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use crate::schema::receipts;

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
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
pub struct PostReceipt<'a>{
  pub user_id: &'a i32,
  pub store: &'a str,
  pub date_bought: &'a NaiveDateTime,
  pub subtotal: &'a f32,
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
