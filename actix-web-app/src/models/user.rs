use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub user_id: String,
    pub phone: String,
    pub age: u32,
    pub name: String,
    pub token: String,
    pub sms_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegisterInfo {
    pub name: String,
    pub age: i32,
    pub phone: String,
    pub verification_code: String,
}


