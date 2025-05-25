```bash
docker compose up --build --force-recreate app
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