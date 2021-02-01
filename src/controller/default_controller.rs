use actix_web::{Responder, web};
use serde::Serialize;
use crate::mysql;
use sqlx::query_as;
use crate::dotenv_handler;

#[derive(Serialize)]
struct ResponseModel {
    message: String,
    api_version: String,
    active_keys: i32,
    licensed_products: Vec<Products>
}

struct ActiveKeys {
    id: i32
}

#[derive(Serialize)]
struct Products {
    name: String,
    information: String,
    licensed_by: String
}

pub async fn response() -> impl Responder {
    let conn = mysql::get_connection().await.unwrap();
    let keys = query_as!(ActiveKeys, "SELECT `id` FROM `product_keys`")
        .fetch_all(&conn).await.unwrap();
    let products = query_as!(Products, "SELECT `name`, `information`, `licensed_by` FROM `products`")
        .fetch_all(&conn).await.unwrap();
    let api_version = dotenv_handler::load_param("API_VERSION");
    web::HttpResponse::Ok()
        .json(ResponseModel{
            message: "License server is running...".to_string(),
            api_version,
            active_keys: keys.len() as i32,
            licensed_products: products
        })
}