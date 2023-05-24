use std::error::Error;
use std::time::Duration;

use tokio::time;

use crate::model::pulsar::Instance;

pub async fn check(instance: Instance) {
    let mut interval = time::interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        log::info!("pulsar consumer abnormal check start");
        match check_internal(instance.clone()).await {
            Ok(_) => {
                log::info!("pulsar consumer abnormal check success");
            }
            Err(e) => {
                log::error!("pulsar consumer abnormal check error: {}", e);
            }
        }
    }
}

async fn check_internal(instance: Instance) -> Result<(), Box<dyn Error>> {
    Ok(())
}
