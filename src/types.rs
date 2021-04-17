use sqlx::{Pool, Postgres};

pub type PostgresPool = Pool<Postgres>;
