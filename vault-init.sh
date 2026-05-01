#!/bin/sh
export VAULT_ADDR=http://vault:8200
sleep 10
vault login myroot
vault secrets enable -path=secret kv-v2 || true
vault kv put secret/mssql host="localhost" port=1433 database="mydemodb" username="myuser" password="MyPassword123!"
vault kv put secret/jwt secret="super-secret-key-for-krungsri-2026" issuer="my-rust-app" expiration="24h"
echo "✅ Vault Bootstrapped: Done!"
