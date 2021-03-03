use actix_web::{Responder, web};
use sqlx::{query, query_as};
use serde::{Serialize, Deserialize};
use crate::mysql;
use crate::dotenv_handler;
use crate::models::session::Session;


#[derive(Deserialize)]
pub struct GenerateKeyRequest {
    secret: String,
    product_name: String
}

#[derive(Serialize)]
struct ResponseModel {
    message: String,
    alert: String,
    status: String,
    http_status: i16,
    key: String
}

#[derive(Serialize, Deserialize)]
struct ProductStruct {
    id: i32,
    product_key: String,
    product_id: i32,
    status: String,
    hardware_ids: String
}

#[derive(Serialize, Deserialize)]
struct ProductID {
    id: i32
}

// controller
pub async fn response(info: web::Query<GenerateKeyRequest>) -> impl Responder {

    // check if secret is correct
    if &info.secret != &dotenv_handler::load_param("ADMIN_SECRET") {
        return web::HttpResponse::Ok()
            .json(ResponseModel {
                message: "your admin secret is ot correct".to_string(),
                alert: "alert alert-danger".to_string(),
                status: "ok".to_string(),
                http_status: 200,
                key: "null".to_string()
            });
    }

    // generates new unique key
    let conn = mysql::get_connection().await.unwrap();
    let product_id: ProductID = query_as!(ProductID, "SELECT `id` FROM `products` WHERE `name`=?", &info.product_name)
        .fetch_one(&conn).await.unwrap();

    let mut found = false;

    let mut key: String = "null".to_string();

    while !found {
        let mut _key = generate_key();
        let st =  query_as!(ProductStruct, "SELECT `id`, `product_key`, `product_id`, `status`, `hardware_ids` FROM `product_keys` WHERE `product_key`=? AND `product_id`=?", _key, &product_id.id)
            .fetch_all(&conn).await.unwrap();
        if st.len() > 0 {
            key = _key;
            found = true;
        }
    }

    query!("INSERT INTO `product_keys` (`id`, `product_key`, `product_id`, `status`, `hardware_ids`) VALUES (NULL, ?, ?, 'sleeping', 'null');", &key, &product_id.id)
        .execute(&conn).await.unwrap();

    return web::HttpResponse::Ok()
        .json(ResponseModel {
            message: "successfully created new product-key".to_string(),
            alert: "alert alert-success".to_string(),
            status: "ok".to_string(),
            http_status: 200,
            key
        })
}

// key generation util
fn generate_key() -> String {
    let mut key_parts: String = "".to_string();
    for i in 0..5 {
        if i == 0 {
            key_parts += &*Session::generate_session_cryptography(4);
            continue;
        }
        key_parts += &*("-".to_owned() + &*Session::generate_session_cryptography(4));
    }
    return key_parts;
}