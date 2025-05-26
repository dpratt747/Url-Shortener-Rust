// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        long_url -> Text,
        #[max_length = 255]
        short_url -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}
