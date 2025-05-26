```bash
docker compose up --build --force-recreate app

docker-compose down --rmi all --volumes
```

[Diesel](https://diesel.rs/)

### Install diesel CLI:
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
```

### Commands:
```bash
diesel setup
diesel migration generate create_urls
diesel migration run --database-url=postgres://postgres:postgres@127.0.0.1/url-shortener-db
diesel migration redo
```

----

```rust
match e {
  ServiceError::StorageError(storage_error) => {
      match storage_error {
          StorageError::ConnectionFailed(msg) => {
              println!("{msg}");
          }
          other => println!("{:?}", other),
      }
  }  
};
```


```text
create a view that lists only valid entries and query that view instead of directly querying the users table

new migration:
CREATE VIEW valid_urls AS
SELECT *
FROM urls
WHERE created_at >= NOW() - INTERVAL '30 minutes';


tldr: inserts will write to the urls table all other queries shoudl read from the valid_urls table|view

Add triggers so that inserts and deletes can be done on the view:


```

```bash
diesel print-schema --database-url=postgres://postgres:postgres@127.0.0.1/url-shortener-db > src/schema.rs
```