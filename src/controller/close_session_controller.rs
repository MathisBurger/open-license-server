use actix_web::{Responder, web};
use serde::{Serialize, Deserialize};
use sqlx::query;
use crate::mysql;

#[derive(Deserialize)]
pub struct CloseSessionRequest {
    pub product_key: String,
    pub session_key: String,
    pub session_token: String
}

#[derive(Serialize)]
struct ResponseModel {
    message: String,
    alert: String,
    status: String,
    http_status: i16,
}

pub async fn response(info: web::Query<CloseSessionRequest>) -> impl Responder {
    let conn = mysql::get_connection().await.unwrap();
    let status =  query!("DELETE FROM `sessions` WHERE `product_key`=? AND `session_key`=? AND `session_token`=?", &info.product_key, &info.session_key, &info.session_token)
        .execute(&conn).await.is_ok();
    conn.close();
    if status {
        return web::HttpResponse::Ok()
            .json(ResponseModel {
                message: "successfully closed session".to_string(),
                alert: "alert alert-success".to_string(),
                status: "ok".to_string(),
                http_status: 200
            });
    } else {
        return web::HttpResponse::Ok()
            .json(ResponseModel {
                message: "error while closing session".to_string(),
                alert: "alert alert-danger".to_string(),
                status: "ok".to_string(),
                http_status: 200
            });
    }
}