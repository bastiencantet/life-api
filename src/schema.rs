// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        image -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
