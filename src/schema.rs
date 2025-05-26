// Take the tables from the generated schema_examples.rs (
//  diesel migration run --database-url=postgres://postgres:postgres@127.0.0.1/url-shortener-db
// )

diesel::table! {
    valid_urls (id) {
        id -> Int4,
        long_url -> Text,
        #[max_length = 255]
        short_url -> Varchar,
        created_at -> Timestamptz,
    }
}
