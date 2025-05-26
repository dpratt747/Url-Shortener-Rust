//

// schema for the urls view table
diesel::table! {
    valid_urls (id) {
        id -> Int4,
        long_url -> Text,
        #[max_length = 255]
        short_url -> Varchar,
        created_at -> Timestamptz,
    }
}