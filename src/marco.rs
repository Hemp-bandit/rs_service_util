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

