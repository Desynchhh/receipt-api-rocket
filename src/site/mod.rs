use rocket::*;
use std::fs;
use std::collections::HashMap;
use rocket_dyn_templates::{ Template };
use rocket::serde::{ Serialize, Deserialize, json };

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub price: f32,
    pub discount: Option<f32>,
    pub contributors: Vec<String>,
}


#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[get("/receipt/<id>")]
fn receipt(id: usize) -> Template {
    let receipts:Vec<ReceiptEntry> = json::from_str(&fs::read_to_string("test.json").unwrap()).unwrap();

    let receipt: ReceiptEntry = receipts.into_iter().filter(|r| r.id == id).nth(0).unwrap();

    let context = HashMap::new().insert("receipt", receipt);

    Template::render("receipt", context)

}

#[get("/receipts")]
fn receipts() -> Template {
    let receipts:Vec<ReceiptEntry> = json::from_str(&fs::read_to_string("test.json").unwrap()).unwrap();
    let context = HashMap::new().insert("receipts", receipts).unwrap();
    Template::render("receipts", context)
}


pub fn routes() -> Vec<rocket::Route> {
    routes![
        index,
        receipts,
        receipt
    ]
}