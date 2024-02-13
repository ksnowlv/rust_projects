use serde::{Serialize, Deserialize};
use sqlx::{Row, FromRow};

#[derive(Debug, FromRow,Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub userid: String,
    pub phone: String,
    pub age: i32,
    pub name: String,
    pub token: String,
    pub verification_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterInfo {
    pub name: String,
    pub age: i32,
    pub phone: String,
    pub verification_code: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenQuery {
    pub token: String,
}


