use serde::{Serialize};

#[derive(Serialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String
}
