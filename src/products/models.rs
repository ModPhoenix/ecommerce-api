use crate::types::PostgresPool;
use anyhow::Result;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct ProductInput {
    pub name: String,
    pub price: i64,
    pub origin: String,
}

#[derive(Serialize, FromRow, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i64,
    pub origin: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl Product {
    pub async fn find_all(pool: &PostgresPool) -> Result<Vec<Product>> {
        let product = sqlx::query_as!(
            Product,
            r#"
              SELECT id, name, price, origin, updated_at, created_at
                  FROM products
              ORDER BY updated_at
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(product)
    }

    pub async fn find_by_id(id: i32, pool: &PostgresPool) -> Result<Product> {
        let product = sqlx::query_as!(
            Product,
            r#"
              SELECT * FROM products WHERE id = $1
            "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(product)
    }

    pub async fn create(input: ProductInput, pool: &PostgresPool) -> Result<Product> {
        let mut tx = pool.begin().await?;
        let product = sqlx::query_as!(
            Product,
            r#"
              INSERT INTO products (name, price, origin) VALUES ($1, $2, $3)
                RETURNING id, name, price, origin, updated_at, created_at
            "#,
            input.name,
            input.price,
            input.origin
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await?;

        Ok(product)
    }

    pub async fn update(id: i32, input: ProductInput, pool: &PostgresPool) -> Result<Product> {
        let mut tx = pool.begin().await.unwrap();
        let product = sqlx::query_as!(
            Product,
            r#"
              UPDATE products SET name = $1, price = $2, origin = $3 WHERE id = $4
                RETURNING id, name, price, origin, updated_at, created_at
            "#,
            input.name,
            input.price,
            input.origin,
            id
        )
        .fetch_one(&mut tx)
        .await?;
        tx.commit().await.unwrap();

        Ok(product)
    }

    pub async fn delete(id: i32, pool: &PostgresPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let result = sqlx::query_as!(Product, "DELETE FROM products WHERE id = $1", id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(result.rows_affected())
    }
}
