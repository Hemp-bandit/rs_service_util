use redis::{aio::MultiplexedConnection, Client};

pub struct RedisTool {
    pub conn: MultiplexedConnection,
}

impl RedisTool {
    pub async fn new(redis_url: String) -> Self {
        log::info!("redis_url {redis_url}");
        let client = Client::open(redis_url).unwrap(); // not recommended
        let conn = client.get_multiplexed_tokio_connection().await;
        match conn {
            Err(err) => {
                let detail = err.detail().unwrap();
                panic!("redis connection err {detail}");
            }
            Ok(conn) => RedisTool { conn },
        }
    }
}
