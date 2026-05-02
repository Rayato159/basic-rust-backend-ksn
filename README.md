# Testing

Install test lib:

```bash
cargo install cargo-tarpaulin
```

Test command:

```bash
cargo tarpaulin --out xml
```

# Container build

```bash
podman build -t basic-rust-backend-ksn:v1.0.0 -f ./Dockerfile
```

# Start on Podman

```bash
podman run --name basic-rust-backend-ksn -p 8080:8080 \
    -e STAGE=Local \
    -e SERVER_PORT=8080 \
    -e SERVER_BODY_LIMIT=10 \
    -e SERVER_TIMEOUT=90 \
    -e DATABASE_URL=postgres://postgres:YUPtqb49xKzdKpe@34.143.235.1:5432/quests_tracker_db \
    -e JWT_SECRET=67 \
    -d basic-rust-backend-ksn:v1.0.0
```

# Vault Secret

MSSQL
```json
{
  "host": "127.0.0.1",
  "port": 1433,
  "database": "mydemodb",
  "username": "sa",
  "password": "StrongPassword123!"
}
```

JWT
```json
{
  "secret": "super-secret-key-for-krungsri-2026"
}
```

# API Docs
`http://localhost:8080/swagger-ui`
