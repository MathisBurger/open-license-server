use sqlx::{query_as, query};
use crate::mysql;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::read;
use std::borrow::Borrow;
use serde::Serialize;


#[derive(Serialize)]
pub struct Session {
    product_key: String,
    session_key: String,
    session_token: String
}

impl Session {
    pub async fn exists_for_key(key: &String) -> bool {
        let conn = mysql::get_connection().await.unwrap();
        let sessions = query_as!(Session, "SELECT `product_key`, `session_key`, `session_token` FROM `sessions` WHERE `product_key`=?", key)
            .fetch_all(&conn).await;
        conn.close();
        if sessions.is_err() {
            return false;
        }
        return sessions.unwrap().len() > 0;
    }

    pub async fn create_session(key: &String) -> Self {
        let conn = mysql::get_connection().await.unwrap();
        let session_key = Self::generate_session_cryptography(64);
        let session_token = Self::generate_session_cryptography(32);
        query!("INSERT INTO `sessions` (`id`, `product_key`, `session_key`, `session_token`) VALUES (NULL, ?, ?, ?);", key, session_key, session_token)
            .execute(&conn).await.unwrap();
        return Session{
            product_key: key.clone(),
            session_key,
            session_token
        }
    }

    pub fn empty_session() -> Self {
        return Session {
            product_key: "null".to_string(),
            session_key: "null".to_string(),
            session_token: "null".to_string()
        }
    }


    fn generate_session_cryptography(len: usize) -> String {
        let random: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        return random;
    }
}