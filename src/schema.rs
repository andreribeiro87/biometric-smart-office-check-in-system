// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        fingerprint -> Bytea,
    }
}
