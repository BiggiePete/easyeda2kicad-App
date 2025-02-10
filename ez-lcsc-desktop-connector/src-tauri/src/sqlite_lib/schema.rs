// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        title -> Text,
        dir -> Text,
    }
}
