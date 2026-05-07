# Tech Stacks
- Axum
- MSSQL + Terbirus
- Vault
- JWT
- Argon2 Hashing
- Mockall

# Start Project In Dev Mode

```
cargo run --bin server
```

# Manaul Migration

```
cargo run --bin migration
```

# Testing

**Install test lib:**

```bash
cargo install cargo-tarpaulin
```

**Test command:**

By project
```bash
cargo tarpaulin --out html
```

By package
```bash
cargo tarpaulin --package <pakcage_name> --out html
```

# Container (Podman)

**Build:**

```bash
podman build -t basic-rust-backend-ksn:v1.0.0 -f ./Dockerfile
```

**Start:**

```bash
podman run --name basic-rust-backend-ksn -p 8080:8080 \
    -e STAGE="Dev" \
    -e SERVER_PORT=8080 \
    -e SERVER_BODY_LIMIT=10 \
    -e SERVER_TIMEOUT=90 \
    -e VAULT_ADDRESS="http://localhost:8200" \
    -e VAULT_TOKEN="myroot" \
    -d basic-rust-backend-ksn:v1.0.0
```

# Vault Secret

**Token:** `myroot`

**MSSQL:**

```json
{
  "host": "127.0.0.1",
  "port": 1433,
  "database": "mydemodb",
  "username": "sa",
  "password": "StrongPassword123!"
}
```

**JWT:**

```json
{
  "expiration": "24h",
  "issuer": "my-rust-app",
  "secret": "super-secret-key-for-krungsri-2026"
}
```

# API Docs

```text
http://localhost:8080/swagger-ui
```

# SQL
```sql
CREATE DATABASE mydemodb;
GO
CREATE LOGIN myuser WITH PASSWORD = 'MyPassword123!', CHECK_POLICY = OFF;
GO
USE mydemodb;
GO
CREATE USER myuser FOR LOGIN myuser;
GO
ALTER ROLE db_owner ADD MEMBER myuser;
GO
```