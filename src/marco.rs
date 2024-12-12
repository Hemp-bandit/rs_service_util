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



#[macro_export]
macro_rules! redis_conn {
    () => {
        async {
            let rds = REDIS.get().expect("msg");
            rds.conn.clone()
        }
    };
}
