cargo install sqlx-cli --no-default-features --features postgres
sqlx database create
sqlx migrate run