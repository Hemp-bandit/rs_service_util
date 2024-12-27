
#[macro_export]
macro_rules! http_client {
    () => {{
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_str("application/json;charset=utf8").unwrap(),
        );
        headers.insert(
            "service_call",
            reqwest::header::HeaderValue::from_str("store_service").unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("build client faille");
        client
    }};
}

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
            let rds = crate::REDIS.get().expect("msg");
            rds.conn.clone()
        }
    };
}

///
/// 获取 transaction
/// *注意*: 在使用后尽快drop tx，否则在多个函数调用中多次调用 `RB.acquire()`  会卡住
///
/// ```
///let tx = transaction!().await;
///
/// let res: Option<()> = tx.query_decode("sql", []).await.expect("msg");
///
/// drop(tx);
///
/// ```
///
#[macro_export]
macro_rules! transaction {
    () => {
        async {
            let tx = crate::RB.acquire_begin().await.unwrap();
            let tx = tx.defer_async(|ex| async move {
                if ex.done() {
                    log::info!("transaction [{}] complete.", ex.tx_id);
                } else {
                    let r = ex.rollback().await;
                    if let Err(e) = r {
                        log::error!("transaction [{}] rollback fail={}", ex.tx_id, e);
                    } else {
                        log::info!("transaction [{}] rollback", ex.tx_id);
                    }
                }
            });
            log::info!("transaction [{}] start", tx.tx_id());
            tx
        }
    };
}
