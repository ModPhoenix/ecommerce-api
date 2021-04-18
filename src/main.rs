use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod products;
pub mod types;

// API overview
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to E-commerce API.
        Available routes:
        GET /products -> list of all products
        POST /products -> create new product, example: { "name": "Apple", "price": "1", origin: "ukraine" }
        GET /products/{id} -> get one product with requested id
        PUT /products/{id} -> update product with requested id, example: { "name": "Apple", "price": "3", origin: "ukraine" }
        DELETE /products/{id} -> delete product with requested id
    "#
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("environment variable: DATABASE_URL");
    let host = env::var("HOST").expect("environment variable: HOST");
    let port = env::var("PORT").expect("environment variable: PORT");
    let addr = format!("{}:{}", host, port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create connections pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .service(
                web::scope("/api").service(web::scope("/v1").configure(products::handlers::config)),
            )
    })
    .bind(addr)?
    .run()
    .await
}
