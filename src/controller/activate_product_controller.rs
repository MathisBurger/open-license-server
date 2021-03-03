use actix_web::{Responder, web};
use serde::{Serialize, Deserialize};
use sqlx::query_as;
use crate::models::session;
use std::time::{SystemTime, UNIX_EPOCH};

// request struct
#[derive(Deserialize)]
pub struct ActivateProductRequest {
    pub product_key: String
}


#[derive(Serialize)]
struct ResponseModel {
    status: bool,
    session: session::Session,
    timestamp: u64
}


// controller
pub async fn response(info: web::Query<ActivateProductRequest>) -> impl Responder {

    // check if a session for product-key exists
    let exists = session::Session::exists_for_key(&info.product_key).await;

    // get unix timestamp
    let unix_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    if exists {
        return web::HttpResponse::Ok()
            .json(ResponseModel{
                status: false,
                session: session::Session::empty_session(),
                timestamp: unix_now
            });
    }

    // creates session for key
    let s = session::Session::create_session(&info.product_key).await;

    return web::HttpResponse::Ok()
        .json(ResponseModel{
            status: true,
            session: s,
            timestamp: unix_now
        })
}