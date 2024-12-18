use redis::{aio::ConnectionManager, Client};

pub struct RedisTool {
    pub conn: ConnectionManager,
}

impl RedisTool {
    pub async fn new(redis_url: String) -> Self {
        log::info!("redis_url {redis_url}");
        let client = Client::open(redis_url).unwrap(); // not recommended
        let manage = client.get_connection_manager().await.expect("msg");
        RedisTool { conn: manage }
    }
}
