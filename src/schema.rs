// @generated automatically by Diesel CLI.

diesel::table! {
    account_statuss (id) {
        id -> Integer,
        name -> Text,
        slug -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_types (id) {
        id -> Integer,
        name -> Text,
        slug -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        pswd -> Text,
        user_type_id -> Integer,
        account_status_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(users -> account_statuss (account_status_id));
diesel::joinable!(users -> user_types (user_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_statuss,
    sessions,
    user_types,
    users,
);
