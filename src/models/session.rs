use sqlx::{query_as, query};
use crate::mysql;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::fs::read;
use std::borrow::Borrow;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
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
        conn.close();
        return Session{
            product_key: key.clone(),
            session_key,
            session_token
        }
    }

    pub async fn update_session_token(old: &Self) -> Self {
        let conn = mysql::get_connection().await.unwrap();
        let new_token = Self::generate_session_cryptography(32);
        let exists = query_as!(Self, "SELECT `product_key`, `session_key`, `session_token` FROM `sessions` WHERE `product_key`=? AND `session_key`=? AND `session_token`=?;",
                              &old.product_key, &old.session_key, &old.session_token)
            .fetch_all(&conn).await.unwrap();
        if exists.len() == 0 {
            conn.close();
            return Self::empty_session();
        }
        query!("UPDATE `sessions` SET `session_token`=? WHERE `product_key`=? AND `session_key`=? AND `session_token`=?;",
              new_token, &old.product_key, &old.session_key, &old.session_token)
            .execute(&conn).await.unwrap();
        conn.close();
        return Self::new(&old.product_key, &old.session_key, &new_token);
    }

    pub fn new(product_key: &String, session_key: &String, session_token: &String) -> Self {
        return Session {
            product_key: product_key.clone(),
            session_key: session_key.clone(),
            session_token: session_token.clone()
        };
    }

    pub fn empty_session() -> Self {
        return Session {
            product_key: "null".to_string(),
            session_key: "null".to_string(),
            session_token: "null".to_string()
        }
    }


    pub fn generate_session_cryptography(len: usize) -> String {
        let random: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect();
        return random;
    }
}