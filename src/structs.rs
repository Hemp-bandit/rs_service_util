use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedisLoginData {
    pub auth: u64,
    pub last_login_time: i64,
    pub name: String,
    pub id: i32,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename = "Enum")]
pub enum Status {
    ACTIVE = 1,
    DEACTIVE = 0,
}

impl Status {
    pub fn from(val: i8) -> Status {
        match val {
            0 => Status::DEACTIVE,
            1 => Status::ACTIVE,
            _ => Status::DEACTIVE,
        }
    }
}
