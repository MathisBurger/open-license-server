use dotenv::dotenv;
use std::env;

pub fn load_param(name: &str) -> String {
    dotenv().ok();
    for (key, value) in env::vars() {
        if key == name.to_string() {
            return value;
        }
    }
    return "not found".to_string();
}