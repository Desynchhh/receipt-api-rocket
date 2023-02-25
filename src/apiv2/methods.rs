use diesel::prelude::*;
use crate::schema::{ users, items, item_contributors, receipts, user_friends };
use crate::db::{
    establish_connection,
    models::{
        users::{ User, NewUser },
        items::{ Item, NewItem },
        item_contributors::{ ItemContributor, NewItemContributor },
        receipts::{ Receipt, NewReceipt, UpdateReceipt },
        user_friends::{ UserFriend, NewUserFriend },
    }
};

pub enum UserGetMethod {
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

pub fn get_user(identifier: UserGetMethod) -> Result<User, diesel::result::Error> {
    let connection = &mut establish_connection();
    let user = match identifier {
        UserGetMethod::Email(email) => {
            users::table
                .filter(users::email.eq(email))
                .first::<User>(connection)
        },
        UserGetMethod::Id(id) => {
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

    let _ = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new user to db");

    users::table
        .filter(users::email.eq(new_user.email))
        .first(connection)
        .expect("Error loading user after creation")
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

pub fn create_receipt(new_receipt: NewReceipt) -> Result<bool, diesel::result::Error> {
    let connection = &mut establish_connection();

    let _ = diesel::insert_into(receipts::table)
        .values(&new_receipt)
        .execute(connection)?;
    Ok(true)
}

pub fn update_receipt(id: i32, update_receipt: UpdateReceipt) -> Result<Receipt, diesel::result::Error> {
    // let receipt = get_receipt(id)?;
    let connection = &mut establish_connection();
    diesel::update(receipts::table)
        .filter(receipts::id.eq(id))
        .set(update_receipt)
        .execute(connection)?;
    let receipt = get_receipt(id)?;
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
