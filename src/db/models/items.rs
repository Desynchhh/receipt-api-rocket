use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::items;

#[derive(Queryable)]
pub struct Item {
  pub id: i32,
  pub receipt_id: i32,
  pub product: String,
  pub price: f32,
  pub discount: Option<f32>,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub is_deleted: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
  pub receipt_id: &'a i32,
  pub product: &'a str,
  pub price: &'a f32,
  pub discount: &'a f32,
}

#[derive(AsChangeset)]
#[diesel(table_name = items)]
pub struct UpdateItem {
  pub receipt_id: Option<i32>,
  pub product: Option<String>,
  pub price: Option<f32>,
  pub discount: Option<f32>,
  pub updated_at: Option<NaiveDateTime>,
  pub is_deleted: Option<bool>,
}
