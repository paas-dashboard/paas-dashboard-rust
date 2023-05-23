use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Instance {
    pub name: String,
    pub host: String,
    pub web_port: u16,
    pub tcp_port: u16,
}
