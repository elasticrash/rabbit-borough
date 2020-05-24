use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct JSONConfiguration {
    pub connection: ConnectionProperties,
    pub binding: BindingProperties,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConnectionProperties {
    pub host: String,
    pub port: i32,
    pub vhost: String,
    pub username: String,
    pub password: String,
    pub heartbeat: i32,
    pub connection_timeout: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BindingProperties {
    pub queue: String,
    pub exchange: String,
    pub routing_key: String,
}
