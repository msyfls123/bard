use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub is_admin: bool,
}
