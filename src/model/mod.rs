use std::collections::HashMap;
use std::env;

pub mod pulsar;

pub struct Config {
    pub pulsar_instances: HashMap<String, pulsar::Instance>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            pulsar_instances: HashMap::new(),
        }
    }

    pub fn add_pulsar_instance(&mut self, key: String, instance: pulsar::Instance) {
        self.pulsar_instances.insert(key, instance);
    }

    pub fn get_pulsar_instance(&self, key: &String) -> Option<&pulsar::Instance> {
        self.pulsar_instances.get(key)
    }

    pub fn delete_pulsar_instance(&mut self, key: &String) -> Option<pulsar::Instance> {
        self.pulsar_instances.remove(key)
    }
}

pub fn create_config_from_env() -> Config {
    let mut config = Config::new();

    for (key, value) in env::vars() {
        if key.starts_with("PD_PULSAR_") {
            let segments: Vec<&str> = key.split('_').collect();
            if segments.len() == 4 {
                let name = segments[2].to_string();
                match segments[3] {
                    "HOST" => {
                        let instance = pulsar::Instance {
                            name: name.clone(),
                            host: value,
                            web_port: env::var(format!("PD_PULSAR_{}_WEB_PORT", name)).unwrap_or_default().parse().unwrap_or(0),
                            tcp_port: env::var(format!("PD_PULSAR_{}_TCP_PORT", name)).unwrap_or_default().parse().unwrap_or(0),
                        };
                        config.add_pulsar_instance(name, instance);
                    },
                    _ => {}
                }
            }
        }
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_pulsar_instance() {
        let mut config = Config::new();
        let instance = pulsar::Instance {
            name: "test".to_string(),
            host: "localhost".to_string(),
            web_port: 8080,
            tcp_port: 6650,
        };

        // Add instance to the config
        config.add_pulsar_instance("test".to_string(), instance.clone());

        // Test get_pulsar_instance method
        match config.get_pulsar_instance(&"test".to_string()) {
            Some(returned_instance) => {
                assert_eq!(returned_instance.name, instance.name);
                assert_eq!(returned_instance.host, instance.host);
                assert_eq!(returned_instance.web_port, instance.web_port);
                assert_eq!(returned_instance.tcp_port, instance.tcp_port);
            },
            None => panic!("Instance not found!"),
        }
    }

    #[test]
    fn test_delete_pulsar_instance() {
        let mut config = Config::new();
        let instance = pulsar::Instance {
            name: "test".to_string(),
            host: "localhost".to_string(),
            web_port: 8080,
            tcp_port: 6650,
        };

        // Add instance to the config
        config.add_pulsar_instance("test".to_string(), instance.clone());

        // Test delete_pulsar_instance method
        match config.delete_pulsar_instance(&"test".to_string()) {
            Some(deleted_instance) => {
                assert_eq!(deleted_instance.name, instance.name);
                assert_eq!(deleted_instance.host, instance.host);
                assert_eq!(deleted_instance.web_port, instance.web_port);
                assert_eq!(deleted_instance.tcp_port, instance.tcp_port);
            },
            None => panic!("Instance not found!"),
        }

        // Test if instance was deleted
        let deleted_instance = config.get_pulsar_instance(&"test".to_string());
        assert!(deleted_instance.is_none(), "Instance was not deleted");
    }
}
