use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedisLoginData {
    pub auth: u64,
    pub last_login_time: i64,
    pub name: String,
    pub id: i32,
}
