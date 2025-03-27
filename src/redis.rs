use std::time::Duration;
use redis::{aio::{ConnectionManager, ConnectionManagerConfig}, Client};

pub struct RedisTool {
    pub conn: ConnectionManager,
}

impl RedisTool {
    pub async fn new(redis_url: String) -> Self {
        log::info!("redis_url {redis_url}");
        let client = Client::open(redis_url).unwrap(); // not recommended
        let mut config = ConnectionManagerConfig::new();
        config =config.set_connection_timeout(Duration::from_secs(30));
        let manage = client.get_connection_manager_with_config(config).await.expect("msg");
        RedisTool { conn: manage }
    }
}
