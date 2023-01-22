use serde_json;
use std::fs;
use super::models::ReceiptEntry;

const PATH_TO_RECEIPTS_FILE:&str = "test.json";

pub fn get_all_receipts() -> Option<Vec<ReceiptEntry>> {
    let file_contents = fs::read_to_string(PATH_TO_RECEIPTS_FILE);
    match file_contents {
        Err(e) => {
            println!("{}", e);
            return None;
        },
        Ok(contents) => {
            let json: Result<Vec<ReceiptEntry>, serde_json::Error> = serde_json::from_str(&contents);
            match json {
                Err(e) => {
                    println!("{}", e);
                    return None;
                },
                Ok(receipts) => Some(receipts)
            }
        }
    }
}


pub fn write_receipt_file(receipts:&Vec<ReceiptEntry>) -> Result<(), std::io::Error> {
    let contents = serde_json::to_string(receipts).unwrap();
    fs::write(PATH_TO_RECEIPTS_FILE, contents)?;
    Ok(())
}


pub fn create_id(all_receipts:&Vec<ReceiptEntry>) -> usize {  
    let mut max_id = 1;
    for receipt in all_receipts {
        if receipt.id > max_id {
            max_id = receipt.id
        }
    }
    max_id += 1;
    max_id
}
