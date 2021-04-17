# E-commerce API

Tiny E-commerce RESTful API.

## Getting Started

### Setup environment

```sh
git clone https://github.com/ModPhoenix/ecommerce-api.git
cd ecommerce-api
docker-compose up
cp .env.example .env
sqlx database create
sqlx migrate run
cargo run
```

### Run dev server

```sh
cargo run
```

or

```sh
cargo install cargo-watch
cargo watch -x run
```
