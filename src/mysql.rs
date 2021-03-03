use sqlx::{MySqlPool, mysql};
use std::io::Error;
use crate::dotenv_handler;

// creates connection
pub async fn get_connection() -> Result<MySqlPool, Error> {
    Ok(mysql::MySqlPool::connect(&dotenv_handler::load_param("DATABASE_URL")).await.expect("Cannot connect to database"))
}