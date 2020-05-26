#[derive(Clone)]
pub struct ConsumerConfiguration<'a> {
    pub host: &'a str,
    pub port: &'a i32,
    pub vhost: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub heartbeat: &'a i32,
    pub connection_timeout: &'a i32,
    pub queue: &'a str,
    pub exchange: &'a str,
    pub routing_key: &'a str,
    pub connection_retry: &'a u64,
}
