use std::error::Error;
use std::time::Duration;

use tokio::time;

use crate::model::pulsar::Instance;
use crate::util;

pub async fn check(instance: Instance) {
    let mut interval = time::interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        log::info!("pulsar topic split brain check start");
        match check_internal(instance.clone()).await {
            Ok(_) => {
                log::info!("pulsar topic split brain check success");
            }
            Err(e) => {
                log::error!("pulsar topic split brain check error: {}", e);
            }
        }
    }
}

async fn check_internal(instance: Instance) -> Result<(), Box<dyn Error>> {
    let host_list = util::lookup_v4_host(instance.host.as_str()).await?;
    Ok(())
}
