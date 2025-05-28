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
Can pattern match the error types like so:
```rust
Err(e) => match e {
    domain::errors::domain_errors::ServiceError::StorageError(storage_error) => {
        match storage_error {
            domain::errors::domain_errors::StorageError::ConnectionFailed(msg) => {
                Either::Left(HttpResponse::InternalServerError().body(format!("Database connection failed: {}", msg)))
            },
            domain::errors::domain_errors::StorageError::DuplicateEntry(msg) => {
                Either::Left(HttpResponse::InternalServerError().body(format!("Duplicate entry: {}", msg)))
            },
            domain::errors::domain_errors::StorageError::SelectionFailed(msg) => {
                Either::Left(HttpResponse::InternalServerError().body(format!("Selection failed: {}", msg)))
            },
            domain::errors::domain_errors::StorageError::TaskJoinError(msg) => {
                Either::Left(HttpResponse::InternalServerError().body(format!("Task join error: {}", msg)))
            },
            domain::errors::domain_errors::StorageError::OtherDatabaseError(msg) => {
                Either::Left(HttpResponse::InternalServerError().body(format!("Database error: {}", msg)))
            },
        }
    }
}
```



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

```bash
diesel print-schema --database-url=postgres://postgres:postgres@127.0.0.1/url-shortener-db > src/persistence/schema_example.rs
diesel migration revert --database-url=postgres://postgres:postgres@127.0.0.1/url-shortener-db
diesel migration run --database-url=postgres://postgres:postgres@127.0.0.1/url-shortener-db
```

Build and verify:
```bash
docker build -t url_shortener_rust . && cargo test --manifest-path ../url-shortener-cucumber/Cargo.toml
```