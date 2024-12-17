use derive_more::derive::Display;
use redis::{Connection, Client};

#[derive(Debug, Display, Clone)]
pub enum RedisCmd {
    #[display("sismember")]
    Sismember,

    #[display("hexists")]
    Hexists,

    #[display("exists")]
    Exists,

    #[display("smembers")]
    Smembers,

    #[display("HGET")]
    Hget,

    #[display("HSET")]
    Hset,

    #[display("HDEL")]
    Hdel,

    #[display("SADD")]
    Sadd,

    #[display("srem")]
    Srem,

    #[display("get")]
    Get,

    #[display("del")]
    Del,

    #[display("setex")]
    SETEX,
}

pub struct RedisTool {
    pub conn: Connection,
}

impl RedisTool {
    pub async fn new(redis_url: String) -> Self {
        log::info!("redis_url {redis_url}");
        let client = Client::open(redis_url).unwrap(); // not recommended
        let conn: Result<Connection, redis::RedisError> = client.get_connection();
        match conn {
            Err(err) => {
                let detail = err.detail().unwrap();
                panic!("redis connection err {detail}");
            }
            Ok(conn) => RedisTool { conn },
        }
    }
}
