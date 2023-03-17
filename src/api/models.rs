use rocket::serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct ReceiptEntry {
  pub id: usize,
  pub store: String,
  pub date: String,
  pub paid_by: String,
  pub items: Vec<Item>,
  pub subtotal: f32,
  pub contributor_to_pay: HashMap<String, f32>
}

impl ReceiptEntry {
  pub fn from_receipt(id:usize, receipt:Receipt) -> ReceiptEntry {
    ReceiptEntry {
      id,
      store: receipt.store,
      date: receipt.date,
      paid_by: receipt.paid_by,
      items: receipt.items,
      subtotal: receipt.subtotal,
      contributor_to_pay: receipt.contributor_to_pay,
    }
  }
  
  pub fn update_all_values(&mut self, other:Receipt) {
    self.store = other.store;
    self.date = other.date;
    self.paid_by = other.paid_by;
    self.items = other.items;
    self.subtotal = other.subtotal;
    self.contributor_to_pay = other.contributor_to_pay;
  }
}


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Receipt {
  pub store: String,
  pub date: String,
  pub paid_by: String,
  pub items: Vec<Item>,
  pub subtotal: f32,
  pub contributor_to_pay: HashMap<String, f32>
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Item {
  pub name: String,
  pub price: f32,
  pub discount: Option<f32>,
  pub contributors: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct ErrorResponse {
  pub error: String
}
