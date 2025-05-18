```bash
curl --location 'http://localhost:8080/shorten' \
--header 'Content-Type: application/json' \
--data '{
    "longUrl": "https://www.bing.com/search?qs=LT&pq=Rust+google&sk=CSYN1&sc=16-11&q=rust+google&cvid=a826e335a74f4217898d8dae1a259447&gs_lcrp=EgRlZGdlKgYIABAAGEAyBggAEAAYQDIGCAEQRRg5MgYIAhAAGEAyBggDEAAYQDIGCAQQABhAMgYIBRAAGEAyBggGEAAYQDIGCAcQABhAMgYICBAAGEDSAQgxNzk4ajBqNKgCCLACAQ&FORM=ANAB01&PC=U531"
}'
```

```bash
curl --location 'http://localhost:8080/<short url path>'
```

```bash
curl --location 'http://localhost:8080/all'
```

## Swagger page:
http://localhost:8080/swagger-ui/

```bash
cargo run
```