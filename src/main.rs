use dotenv::dotenv;
use actix_web::{HttpServer, App, web, http};
use actix_cors::Cors;

mod dotenv_handler;
mod controller;
mod mysql;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::new().supports_credentials().finish())
            .route("/", web::get().to(controller::default_controller::response))
            .route("/activate-product", web::get().to(controller::activate_product_controller::response))
    })
        .bind("0.0.0.0:".to_owned() + &dotenv_handler::load_param("APPLICATION_PORT"))?
        .run()
        .await
}