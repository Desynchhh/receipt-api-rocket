use rocket::{*, request::FlashMessage};
use std::fs;
use std::collections::HashMap;
use rocket_dyn_templates::{ Template, tera::Context };
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

#[get("/receipt/<id>")]
fn get(id: usize) -> Template {
    let receipts:Vec<ReceiptEntry> = json::from_str(&fs::read_to_string("data.json").unwrap()).unwrap();

    let receipt: ReceiptEntry = receipts.into_iter().filter(|r| r.id == id).nth(0).unwrap();

    let context = HashMap::new().insert("receipt", receipt);

    Template::render("receipts/receipt", context)

}

#[get("/receipts")]
fn get_all() -> Template {
    let data = fs::read_to_string("data.json").unwrap();
    let receipts:Vec<ReceiptEntry> = json::from_str(&data).unwrap();

    let mut context = rocket_dyn_templates::tera::Context::new();
    context.insert("receipts", &receipts);

    Template::render("receipts/receipts", context.into_json())
}

#[get("/receipts/create")]
fn create_get(flash: Option<FlashMessage<'_>>) -> Template {
    let mut context = Context::new();
    if let Some(flash_message) = flash.map(|msg| format!("{}", msg.message())) {
        context.insert("flash_message", &flash_message)
    }
    Template::render("receipts/create", context.into_json())
}

#[post("/receipts/create", data = "<receipt>")]
fn create_post(receipt: json::Json<crate::db::models::receipts::NewReceipt>) -> String {
    let receipt = receipt.into_inner();
    println!("{:?}", receipt);
    receipt.store
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get,
        get_all,
        create_get,
        create_post
    ]
}