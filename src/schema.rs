// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}
