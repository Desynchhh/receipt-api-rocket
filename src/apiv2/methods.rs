use diesel::prelude::*;
use crate::db::models::item_contributors::UpdateItemContributor;
use crate::schema::{ users, items, item_contributors, receipts, user_friends };
use crate::db::{
    establish_connection,
    models::{
        users::{ User, NewUser },
        items::{ Item, NewItem, UpdateItem },
        item_contributors::{ ItemContributor, NewItemContributor },
        receipts::{ Receipt, PostReceipt, UpdateReceipt },
        user_friends::{ UserFriend, NewUserFriend },
    }
};

pub enum GetByField {
    Email(String),
    Id(i32)
}

pub fn get_users() -> Vec<User> {
    let connection = &mut establish_connection();

    let users = users::table
    .load::<User>(connection);
    match users {
        Ok(all_users) => all_users,
        Err(_) => vec![]
    }
}

pub fn get_user(identifier: GetByField) -> Result<User, diesel::result::Error> {
    let connection = &mut establish_connection();
    let user = match identifier {
        GetByField::Email(email) => {
            users::table
                .filter(users::email.eq(email))
                .first::<User>(connection)
        },
        GetByField::Id(id) => {
            users::table
                .filter(users::id.eq(id))
                .first::<User>(connection)
        }
    };

    if let Err(err) = user {
        return Err(err);
    }
    Ok(user.unwrap())
}

pub fn create_user(new_user: NewUser) -> User {
    let connection = &mut establish_connection();

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user to db");

    user
}

pub fn delete_user(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced = diesel::delete(users::table.filter(users::id.eq(id))).execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false
    }
}

pub fn get_all_user_emails() -> Vec<String> {
    let connection = &mut establish_connection();

    let users = users::table
    .select(users::email)
    .load::<String>(connection);
    match users {
        Ok(all_users) => all_users,
        Err(_) => vec![]
    }
}

pub fn get_receipts() -> Vec<Receipt> {
    let connection = &mut establish_connection();

    let receipts = receipts::table
    .load::<Receipt>(connection);
    match receipts {
        Ok(all_receipts) => all_receipts,
        Err(_) => vec![]
    }
}

pub fn get_receipt(id: i32) -> Result<Receipt, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    let receipt = receipts::table
        .filter(receipts::id.eq(id))
        .first::<Receipt>(connection);
    if let Err(err) = receipt {
        return Err(err);
    }
    Ok(receipt.unwrap())
}

pub fn create_receipt(new_receipt: PostReceipt) -> Result<Receipt, diesel::result::Error> {
    let connection = &mut establish_connection();

    let receipt: Receipt = diesel::insert_into(receipts::table)
        .values(&new_receipt)
        .get_result(connection)?;

    Ok(receipt)
}

pub fn update_receipt(id: i32, update_receipt: UpdateReceipt) -> Result<Receipt, diesel::result::Error> {
    let connection = &mut establish_connection();
    let receipt = diesel::update(receipts::table)
        .filter(receipts::id.eq(id))
        .set(update_receipt)
        .get_result(connection)?;
    Ok(receipt)
}

pub fn delete_receipt(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced = diesel::delete(receipts::table.filter(receipts::id.eq(id))).execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false
    }
}

// Batch insert not working for whatever reason
// pub fn create_items(new_items: Vec<NewItem>) -> Result<Vec<Item>, diesel::result::Error> {
//     let connection = &mut establish_connection();
    
//     let items: Result<Vec<Item>, diesel::result::Error> = diesel::insert_into(items::table)
//         .values(&new_items)
//         .get_results(connection);

//     Ok(items)
// }

pub fn create_item(new_item: NewItem) -> Result<Item, diesel::result::Error> {
    let connection = &mut establish_connection();

    let item: Item = diesel::insert_into(items::table)
        .values(new_item)
        .get_result(connection)?;

    Ok(item)
}

pub fn update_item(id: i32, update_item: UpdateItem) -> Result<Item, diesel::result::Error> {
    let connection = &mut establish_connection();
    let item = diesel::update(items::table)
        .filter(items::id.eq(id))
        .set(update_item)
        .get_result(connection)?;
    Ok(item)
}

pub fn delete_item(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced = diesel::delete(items::table.filter(items::id.eq(id))).execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false
    }
}

pub fn get_item(id: i32) -> Result<Item, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    let item = items::table
        .filter(items::id.eq(id))
        .first::<Item>(connection);
    if let Err(err) = item {
        return Err(err);
    }
    Ok(item.unwrap())
}

pub fn get_receipt_items(receipt_id: i32) -> Result<Vec<Item>, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    let items = items::table
        .filter(items::receipt_id.eq(receipt_id))
        .load::<Item>(connection);
    if let Err(err) = items {
        return Err(err);
    }
    Ok(items.unwrap())
}

pub fn create_contributor(new_contributor: NewItemContributor) -> Result<ItemContributor, diesel::result::Error> {
    let connection = &mut establish_connection();

    let contributor: ItemContributor = diesel::insert_into(item_contributors::table)
        .values(&new_contributor)
        .get_result(connection)?;

    Ok(contributor)
}

pub fn update_contributor(id: i32, update_contributor: UpdateItemContributor) -> Result<ItemContributor, diesel::result::Error> {
    let connection = &mut establish_connection();
    let contributor = diesel::update(item_contributors::table)
        .filter(item_contributors::id.eq(id))
        .set(update_contributor)
        .get_result(connection)?;
    Ok(contributor)
}

pub fn delete_contributor(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced = diesel::delete(item_contributors::table.filter(item_contributors::id.eq(id))).execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false
    }
}

pub fn get_contributor(id: i32) -> Result<ItemContributor, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    let contributor = item_contributors::table
        .filter(item_contributors::id.eq(id))
        .first::<ItemContributor>(connection);
    if let Err(err) = contributor {
        return Err(err);
    }
    Ok(contributor.unwrap())
}

pub fn create_friend(new_user_friend: NewUserFriend) -> Result<UserFriend, diesel::result::Error> {
    let connection = &mut establish_connection();

    let user_friend: UserFriend = diesel::insert_into(user_friends::table)
        .values(&new_user_friend)
        .get_result(connection)?;

    Ok(user_friend)
}

pub fn delete_friend(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced = diesel::delete(user_friends::table.filter(user_friends::id.eq(id))).execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false
    }
}

pub fn get_friend(user_id: &i32, friend_id: &i32) -> Result<UserFriend, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    let friend = user_friends::table
        .filter(user_friends::user_id.eq(user_id))
        .filter(user_friends::friend_id.eq(friend_id))
        .first::<UserFriend>(connection);
    if let Err(err) = friend {
        return Err(err);
    }
    Ok(friend.unwrap())
}

pub fn get_friends(user_id: &i32) -> Result<Vec<UserFriend>, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    let friends = user_friends::table
        .filter(user_friends::user_id.eq(user_id))
        .or_filter(user_friends::user_id.eq(user_id))
        .load::<UserFriend>(connection);
    if let Err(err) = friends {
        return Err(err);
    }
    Ok(friends.unwrap())
}

pub fn set_friend_status(user_id:i32, friend_id:i32, request_state:bool) -> Result<UserFriend, diesel::result::Error> {
    let connection = &mut establish_connection();
    
    diesel::update(user_friends::table)
        .filter(user_friends::user_id.eq(user_id))
        .filter(user_friends::friend_id.eq(friend_id))
        .set(user_friends::request_accepted.eq(request_state))
        .get_result(connection)
}
