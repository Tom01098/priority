// @generated automatically by Diesel CLI.

diesel::table! {
    todo (id) {
        id -> Integer,
        title -> Text,
        created_at -> Timestamp,
    }
}
