use actix_web::{Responder, web};
use serde::{Serialize, Deserialize};
use sqlx::query;
use crate::mysql;
use crate::dotenv_handler;

#[derive(Deserialize)]
pub struct AddProductRequest {
    name: String,
    information: String,
    licensed_by: String,
    secret: String,
}

#[derive(Serialize)]
struct ResponseModel {
    message: String,
    alert: String,
    status: String,
    http_status: i16
}


pub async fn response(info: web::Query<AddProductRequest>) -> impl Responder {
    if &info.secret != &dotenv_handler::load_param("ADMIN_SECRET") {
        return web::HttpResponse::Ok()
            .json(ResponseModel {
                message: "your admin secret is ot correct".to_string(),
                alert: "alert alert-danger".to_string(),
                status: "ok".to_string(),
                http_status: 200
            });
    }
    let conn = mysql::get_connection().await.unwrap();
    let status = query!("INSERT INTO `products` (`id`, `name`, `information`, `licensed_by`) VALUES (NULL, ?, ?, ?)", &info.name, &info.information, &info.licensed_by)
        .execute(&conn).await.is_ok();
    conn.close();
    if status {
        return web::HttpResponse::Ok()
            .json(ResponseModel {
                message: "successfully added product".to_string(),
                alert: "alert alert-success".to_string(),
                status: "ok".to_string(),
                http_status: 200
            });
    } else {
        return web::HttpResponse::Ok()
            .json(ResponseModel {
                message: "error while adding product to database".to_string(),
                alert: "alert alert-danger".to_string(),
                status: "ok".to_string(),
                http_status: 200
            })
    }

}