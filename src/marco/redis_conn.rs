///
/// 获取 redis_conn
///
/// ```
///let mut rds = redis_conn!().await;
///
/// let res: Option<()> = rds.get("key").await.expect("msg");
///
/// ```
///
#[macro_export]
macro_rules! redis_conn {
    () => {
        async {
            let rds = REDIS.get().expect("msg");
            rds.conn.clone()
        }
    };
}
