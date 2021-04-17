use crate::{
    products::models::{Product, ProductInput},
    types::PostgresPool,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/products")]
async fn find_all(pool: web::Data<PostgresPool>) -> impl Responder {
    let result = Product::find_all(pool.get_ref()).await;
    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        _ => HttpResponse::BadRequest().body("Error trying to read all products from database"),
    }
}

#[get("/products/{id}")]
async fn find_by_id(id: web::Path<i32>, pool: web::Data<PostgresPool>) -> impl Responder {
    let result = Product::find_by_id(id.into_inner(), pool.get_ref()).await;
    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        _ => HttpResponse::NotFound().body("Product not found"),
    }
}

#[post("/products")]
async fn create(input: web::Json<ProductInput>, pool: web::Data<PostgresPool>) -> impl Responder {
    let result = Product::create(input.into_inner(), pool.get_ref()).await;
    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        _ => HttpResponse::BadRequest().body("Error trying to create new product"),
    }
}

#[put("/products/{id}")]
async fn update(
    id: web::Path<i32>,
    todo: web::Json<ProductInput>,
    pool: web::Data<PostgresPool>,
) -> impl Responder {
    let result = Product::update(id.into_inner(), todo.into_inner(), pool.get_ref()).await;
    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        _ => HttpResponse::NotFound().body("Product not found"),
    }
}

#[delete("/products/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PostgresPool>) -> impl Responder {
    let result = Product::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok().body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::NotFound().body("Product not found")
            }
        }
        _ => HttpResponse::InternalServerError().body("Fail delete product"),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
