use std::env;
use crate::constant::CONFIG;

mod pulsar_consumer_abnormal_checker;
mod pulsar_topic_split_brain_checker;

pub fn init() {
    if let Ok(value) = env::var("PD_PULSAR_CONSUMER_ABNORMAL_CHECK_ENABLE") {
        if value.to_lowercase() == "true" {
            for (_, instance) in CONFIG.pulsar_instances.iter() {
                tokio::task::spawn(async move {
                    pulsar_consumer_abnormal_checker::check(instance.clone()).await
                });
            }
        }
    }
    if let Ok(value) = env::var("PD_PULSAR_TOPIC_SPLIT_BRAIN_CHECK_ENABLE") {
        if value.to_lowercase() == "true" {
            for (_, instance) in CONFIG.pulsar_instances.iter() {
                tokio::task::spawn(async move {
                    pulsar_topic_split_brain_checker::check(instance.clone()).await
                });
            }
        }
    }
}
