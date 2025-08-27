// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        description -> Nullable<Text>,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
