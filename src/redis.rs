use redis::{Client, Connection};
use std::sync::Arc;

pub struct RedisTool {
    pub conn: Arc<Connection>,
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
            Ok(conn) => RedisTool {
                conn: Arc::new(conn),
            },
        }
    }
}
