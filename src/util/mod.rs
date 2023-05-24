use std::env;
use std::error::Error;
use std::net::SocketAddrV4;

pub fn init() {
    let log_level = match env::var("LOG_LEVEL") {
        Ok(level) => {
            if level == "trace" {
                log::LevelFilter::Trace
            } else if level == "debug" {
                log::LevelFilter::Debug
            } else if level == "info" {
                log::LevelFilter::Info
            } else if level == "warn" {
                log::LevelFilter::Warn
            } else if level == "error" {
                log::LevelFilter::Error
            } else {
                log::LevelFilter::Info
            }
        }
        Err(..) => {
            log::LevelFilter::Info
        }
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply().unwrap();
}

pub async fn lookup_v4_host(host: &str) -> Result<Vec<SocketAddrV4>, Box<dyn Error>> {
    match tokio::net::lookup_host(host).await {
        Ok(addresses) => {
            let mut socket_addresses = Vec::new();
            for address in addresses {
                if let std::net::SocketAddr::V4(socket_address) = address {
                    socket_addresses.push(socket_address);
                }
            }
            Ok(socket_addresses)
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn lookup_host(host: &str) -> Result<Vec<std::net::SocketAddr>, Box<dyn Error>> {
    match tokio::net::lookup_host(host).await {
        Ok(addresses) => {
            let mut socket_addresses = Vec::new();
            for address in addresses {
                socket_addresses.push(address);
            }
            Ok(socket_addresses)
        }
        Err(e) => Err(Box::new(e)),
    }
}
