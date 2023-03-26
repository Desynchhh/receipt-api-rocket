use crate::db::{
    establish_connection,
    models::{
        item_contributors::{ItemContributor, NewItemContributor, UpdateItemContributor},
        items::{Item, NewItem, UpdateItem},
        receipts::{PostReceipt, Receipt, UpdateReceipt},
        user_friends::{NewUserFriend, UserFriend},
        users::{NewUser, User, UserDetails},
    },
};
use crate::schema::{item_contributors, items, receipts, user_friends, users};
use diesel::prelude::*;

pub enum GetByField {
    Email(String),
    Id(i32),
}

pub fn get_users() -> Vec<User> {
    let connection = &mut establish_connection();

    let users = users::table.load::<User>(connection);
    match users {
        Ok(all_users) => all_users,
        Err(_) => vec![],
    }
}

pub fn get_user(identifier: GetByField) -> Result<User, diesel::result::Error> {
    let connection = &mut establish_connection();
    match identifier {
        GetByField::Email(email) => users::table
            .filter(users::email.eq(email))
            .first::<User>(connection),
        GetByField::Id(id) => users::table
            .filter(users::id.eq(id))
            .first::<User>(connection),
    }
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
        Err(_) => false,
    }
}

pub fn get_all_user_emails() -> Vec<String> {
    let connection = &mut establish_connection();

    let users = users::table.select(users::email).load::<String>(connection);
    match users {
        Ok(all_users) => all_users,
        Err(_) => vec![],
    }
}

pub fn get_receipts(user_id: &i32) -> Vec<Receipt> {
    let connection = &mut establish_connection();

    let receipts = receipts::table
        // .inner_join(users::table.on(users::id.eq(receipts::user_id)))
        .filter(receipts::user_id.eq(user_id))
        .load::<Receipt>(connection);

    match receipts {
        Ok(all_receipts) => all_receipts,
        Err(_) => vec![],
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

pub fn update_receipt(
    id: i32,
    update_receipt: UpdateReceipt,
) -> Result<Receipt, diesel::result::Error> {
    let connection = &mut establish_connection();
    let receipt = diesel::update(receipts::table)
        .filter(receipts::id.eq(id))
        .set(update_receipt)
        .get_result(connection)?;
    Ok(receipt)
}

pub fn delete_receipt(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced =
        diesel::delete(receipts::table.filter(receipts::id.eq(id))).execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false,
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
        Err(_) => false,
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

pub fn create_contributor(
    new_contributor: NewItemContributor,
) -> Result<ItemContributor, diesel::result::Error> {
    let connection = &mut establish_connection();

    let contributor: ItemContributor = diesel::insert_into(item_contributors::table)
        .values(&new_contributor)
        .get_result(connection)?;

    Ok(contributor)
}

pub fn update_contributor(
    id: i32,
    update_contributor: UpdateItemContributor,
) -> Result<ItemContributor, diesel::result::Error> {
    let connection = &mut establish_connection();
    let contributor = diesel::update(item_contributors::table)
        .filter(item_contributors::id.eq(id))
        .set(update_contributor)
        .get_result(connection)?;
    Ok(contributor)
}

pub fn delete_contributor(id: i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced =
        diesel::delete(item_contributors::table.filter(item_contributors::id.eq(id)))
            .execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false,
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

pub fn delete_friend(user_id: &i32, friend_id: &i32) -> bool {
    let connection = &mut establish_connection();

    let rows_affeced: Result<usize, diesel::result::Error> = diesel::delete(user_friends::table)
        .filter(user_friends::friend_id.eq(user_id))
        .filter(user_friends::user_id.eq(friend_id))
        .filter(user_friends::request_accepted.eq(false))
        .execute(connection);
    match rows_affeced {
        Ok(_num) => true,
        Err(_) => false,
    }
}

pub fn get_friend(user_id: &i32, friend_id: &i32) -> Result<UserFriend, diesel::result::Error> {
    let connection = &mut establish_connection();

    user_friends::table
        .filter(
            user_friends::friend_id
                .eq(friend_id)
                .or(user_friends::user_id.eq(friend_id)),
        )
        .filter(
            user_friends::friend_id
                .eq(user_id)
                .or(user_friends::user_id.eq(user_id)),
        )
        .filter(user_friends::request_accepted.eq(true))
        .first::<UserFriend>(connection)
}

pub fn get_friends(user_id: &i32) -> Result<Vec<UserDetails>, diesel::result::Error> {
    let connection = &mut establish_connection();

    let friends: Result<Vec<UserDetails>, diesel::result::Error> = users::table
        .inner_join(
            user_friends::table.on(user_friends::user_id
                .eq(users::id)
                .or(user_friends::friend_id.eq(users::id))),
        )
        .filter(users::id.ne(user_id))
        .filter(
            user_friends::friend_id
                .eq(user_id)
                .or(user_friends::user_id.eq(user_id)),
        )
        .filter(user_friends::request_accepted.eq(true))
        .select((users::id, users::email, users::first_name, users::last_name))
        .load::<UserDetails>(connection);

    if friends.is_ok() {
        let f = friends.as_ref().unwrap();
        let has_friends = f.len() > 0;

        if !has_friends {
            let user: Result<Vec<UserDetails>, diesel::result::Error> = users::table
                .filter(users::id.eq(user_id))
                .select((users::id, users::email, users::first_name, users::last_name))
                .load::<UserDetails>(connection);
            return user;
        }
    }

    friends
}

pub fn accept_friend_request(
    user_id: &i32,
    friend_id: &i32,
) -> Result<UserFriend, diesel::result::Error> {
    let connection = &mut establish_connection();

    diesel::update(user_friends::table)
        .filter(user_friends::friend_id.eq(user_id))
        .filter(user_friends::user_id.eq(friend_id))
        .set(user_friends::request_accepted.eq(true))
        .get_result(connection)
}

pub fn get_pending_friend_requests(
    user_id: &i32,
) -> Result<Vec<UserDetails>, diesel::result::Error> {
    let connection = &mut establish_connection();

    let friend_requests: Result<Vec<UserDetails>, diesel::result::Error> = users::table
        .inner_join(user_friends::table.on(user_friends::user_id.eq(users::id)))
        .filter(user_friends::friend_id.eq(user_id))
        .filter(user_friends::request_accepted.eq(false))
        .select((users::id, users::email, users::first_name, users::last_name))
        .load::<UserDetails>(connection);

    friend_requests
}
