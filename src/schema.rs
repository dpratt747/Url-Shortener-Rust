// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        #[max_length = 255]
        long_url -> Varchar,
        #[max_length = 255]
        short_url -> Varchar,
        created_at -> Timestamptz,
    }
}
