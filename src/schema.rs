// @generated automatically by Diesel CLI.

diesel::table! {
    item_contributors (id) {
        id -> Integer,
        user_id -> Integer,
        item_id -> Integer,
        created_at -> Date,
        updated_at -> Date,
        is_deleted -> Bool,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        receipt_id -> Integer,
        price -> Float,
        discount -> Nullable<Float>,
        created_at -> Date,
        updated_at -> Date,
        is_deleted -> Bool,
    }
}

diesel::table! {
    receipts (id) {
        id -> Integer,
        user_id -> Integer,
        store -> Text,
        date_bought -> Date,
        subtotal -> Float,
        created_at -> Date,
        updated_at -> Date,
        is_deleted -> Bool,
    }
}

diesel::table! {
    user_friends (id) {
        id -> Integer,
        user_id -> Integer,
        friend_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        email -> Text,
        first_name -> Text,
        last_name -> Text,
        password -> Text,
        api_token -> Nullable<Text>,
        created_at -> Date,
        updated_at -> Date,
        is_deleted -> Bool,
    }
}

diesel::joinable!(item_contributors -> items (item_id));
diesel::joinable!(item_contributors -> users (user_id));
diesel::joinable!(items -> receipts (receipt_id));
diesel::joinable!(receipts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    item_contributors,
    items,
    receipts,
    user_friends,
    users,
);
