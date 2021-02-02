use actix_web::{HttpRequest, web, HttpResponse, Error, Responder, get};
use actix_send_websocket::{Message, WebSocket};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::models::session::Session;



#[derive(Serialize, Deserialize)]
struct ResponseModel {
    status: bool,
    message: String,
    product_key: String,
    session_key: String,
    session_token: String
}

#[get("/check-status")]
pub async fn response(ws: WebSocket) -> impl Responder {
    let (mut stream, res, mut tx) = ws.into_parts();

    actix_web::rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            let result = match msg {
                Message::Text(m) => {
                    let s: Result<Session, serde_json::error::Error> = serde_json::from_str(m.as_str());
                    if s.is_ok() {
                        let el: Session = serde_json::from_str(m.as_str()).unwrap();
                      tx.text(serde_json::to_string(&Session::update_session_token(&el).await).unwrap())
                    } else {
                        tx.text(serde_json::to_string(&Session::empty_session()).unwrap())
                    }

                },
                Message::Close(reason) => {
                    println!("check the resume");
                    let _ = tx.close(reason);
                    break;
                }
                _ => Ok(())
            };
            if result.is_err() {
                println!("Error within socket connection");
                break;
            }
        }
    });
    return res;
}