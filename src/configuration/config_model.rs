use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct JSONConfiguration {
    pub host: String,
    pub port: i32,
    pub vhost: String,
    pub username: String,
    pub password: String,
    pub heartbeat: i32,
    pub connection_timeout: i32,
}
