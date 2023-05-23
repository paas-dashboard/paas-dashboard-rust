use std::time::Duration;
use tokio::time;
use crate::model::pulsar::Instance;

pub async fn check(instance: Instance) {
    let mut interval = time::interval(Duration::from_secs(300));
    loop {
        interval.tick().await;
        log::info!("pulsar topic split brain check start");
    }
}
