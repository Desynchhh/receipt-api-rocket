use crate::db::models::{
    item_contributors::{ItemContributor, NewItemContributor},
    items::{Item, NewItem},
    receipts::{PostReceipt, Receipt},
};
use chrono::NaiveDateTime;
use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    *,
};

use super::{methods, request_guard::JwtToken, HttpPostResponse};

mod utils;

#[post("/receipts/create", data = "<receipt>")]
fn create(
    jwt: JwtToken,
    receipt: Json<NewReceiptObject>,
) -> Json<HttpPostResponse<Receipt, String>> {
    let receipt = receipt.into_inner();
    let logged_in_user = methods::get_user(methods::GetByField::Email(jwt.email)).unwrap();
    let subtotal = utils::calc_subtotal(&receipt.items);

    let new_receipt = PostReceipt {
        user_id: &logged_in_user.id,
        store: &receipt.store,
        date_bought: &receipt.date,
        subtotal: &subtotal,
    };

    let created_receipt = methods::create_receipt(new_receipt);

    if let Err(e) = created_receipt {
        return Json::from(HttpPostResponse::Failure(format!(
            "Something went wrong during receipt creation.\n {}",
            e
        )));
    }
    let created_receipt = created_receipt.unwrap();

    let mut new_items: Vec<NewItem> = Vec::new();
    for item in &receipt.items {
        new_items.append(&mut vec![NewItem {
            receipt_id: &created_receipt.id,
            product: &item.product,
            price: &item.price,
            discount: &item.discount,
        }]);
    }

    let mut created_items: Vec<Item> = vec![];
    for item in new_items {
        let created_item = methods::create_item(item);
        if let Err(e) = created_item {
            return Json::from(HttpPostResponse::Failure(format!(
                "Something went wrong during item creation.\n {}",
                e
            )));
        }
        let created_item = created_item.unwrap();
        created_items.append(&mut vec![created_item]);
    }

    let mut created_contributors: Vec<ItemContributor> = Vec::new();
    for (i, item) in created_items.into_iter().enumerate() {
        let item_contributor_ids = &receipt.items[i].contributor_ids;
        for contributor_id in item_contributor_ids {
            let item_id = &item.id;
            let new_contributor: NewItemContributor = NewItemContributor {
                user_id: contributor_id,
                item_id,
            };
            let created_contributor = methods::create_contributor(new_contributor).unwrap();
            created_contributors.push(created_contributor);
        }
    }

    Json::from(HttpPostResponse::Success(created_receipt))
}

#[get("/receipts")]
fn get_all(jwt: JwtToken) -> Json<HttpPostResponse<Vec<Receipt>, String>> {
    let receipts = super::methods::get_receipts(&jwt.id);

    Json::from(HttpPostResponse::Success(receipts))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create, get_all,]
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct NewItemObject {
    product: String,
    price: f32,
    #[serde(default = "default_discount")]
    discount: f32,
    contributor_ids: Vec<i32>,
}

fn default_discount() -> f32 {
    0.0
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct NewReceiptObject {
    store: String,
    date: NaiveDateTime,
    items: Vec<NewItemObject>,
}
