pub struct ConsumerConfiguration<'a> {
    pub host: &'a str,
    pub port: &'a i32,
    pub vhost: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub heartbeat: &'a i32,
    pub connection_timeout: &'a i32,
}
