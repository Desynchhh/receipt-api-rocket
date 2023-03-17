use self::models::{ErrorResponse, Receipt, ReceiptEntry};
use rocket::serde::json::Json;
use rocket::*;

mod models;
mod utils;

#[get("/")]
fn get_receipts() -> Result<Json<Vec<ReceiptEntry>>, Json<ErrorResponse>> {
  match utils::get_all_receipts() {
    None => Err(Json(ErrorResponse {
      error: "No receipts found.".to_string(),
    })),
    Some(receipts) => Ok(Json(receipts)),
  }
}

#[get("/<id>")]
fn get_receipt(id: usize) -> Result<Json<ReceiptEntry>, Json<ErrorResponse>> {
  match utils::get_all_receipts() {
    None => Err(Json(ErrorResponse {
      error: "No receipts found.".to_string(),
    })),
    Some(receipts) => {
      let receipt = receipts.into_iter().filter(|r| r.id == id).nth(0);
      if receipt.is_none() {
        return Err(Json(ErrorResponse {
          error: format!("No receipt with id {} found.", id),
        }));
      }
      Ok(Json(receipt.unwrap()))
    }
  }
}

#[post("/", format = "application/json", data = "<new_receipt>")]
fn create_receipt(new_receipt: Json<Receipt>) -> Result<Json<ReceiptEntry>, Json<ErrorResponse>> {
  match utils::get_all_receipts() {
    None => Err(Json(ErrorResponse {
      error: "No receipts found.".to_string(),
    })),
    Some(mut receipts) => {
      let id = utils::create_id(&receipts);
      let new_receipt = ReceiptEntry::from_receipt(id, new_receipt.into_inner());
      receipts.push(new_receipt.clone());
      match utils::write_receipt_file(&receipts) {
        Err(e) => Err(Json(ErrorResponse {
          error: format!("{:?}", e),
        })),
        Ok(_) => Ok(Json(new_receipt)),
      }
    }
  }
}

#[put(
  "/<receipt_id>",
  format = "application/json",
  data = "<edited_receipt>"
)]
fn update_receipt(
  receipt_id: usize,
  edited_receipt: Json<Receipt>,
) -> Result<Json<ReceiptEntry>, Json<ErrorResponse>> {
  match utils::get_all_receipts() {
    None => Err(Json(ErrorResponse {
      error: "No receipts found.".to_string(),
    })),
    Some(mut all_receipts) => {
      let edited_receipt = edited_receipt.into_inner();
      let receipt_option = all_receipts.iter().filter(|r| r.id == receipt_id).nth(0);
      if receipt_option.is_none() {
        return Err(Json(ErrorResponse {
          error: format!("No receipt with id {} found.", receipt_id),
        }));
      }
      let mut receipt = receipt_option.unwrap().to_owned();
      receipt.update_all_values(edited_receipt);
      let index = all_receipts
      .iter()
      .position(|r| r.id == receipt.id)
      .unwrap();
      all_receipts[index] = receipt.clone();
      
      match utils::write_receipt_file(&all_receipts) {
        Err(e) => Err(Json(ErrorResponse {
          error: format!("{:?}", e),
        })),
        Ok(_) => Ok(Json(receipt)),
      }
    }
  }
}

#[delete("/<receipt_id>")]
fn delete_receipt(receipt_id: usize) -> Result<Json<Vec<ReceiptEntry>>, Json<ErrorResponse>> {
  match utils::get_all_receipts() {
    None => Err(Json(ErrorResponse {
      error: "No receipts found.".to_string(),
    })),
    Some(receipts) => {
      let total_receipts = receipts.len();
      let receipts: Vec<models::ReceiptEntry> = receipts
      .into_iter()
      .filter(|r| r.id != receipt_id)
      .collect();
      if total_receipts <= receipts.len() {
        return Err(Json(ErrorResponse {
          error: format!("No receipt with id {} found", receipt_id),
        }));
      }
      match utils::write_receipt_file(&receipts) {
        Err(e) => Err(Json(ErrorResponse {
          error: format!("{:?}", e),
        })),
        Ok(_) => Ok(Json(receipts)),
      }
    }
  }
}

pub fn routes() -> Vec<Route> {
  routes![
  get_receipts,
  get_receipt,
  create_receipt,
  update_receipt,
  delete_receipt,
  ]
}
