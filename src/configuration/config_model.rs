use serde::Deserialize;

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct JSONConfiguration {
    pub connection: ConnectionProperties,
    pub binding: BindingProperties,
    pub declare: DeclareProperties,
}

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct ConnectionProperties {
    pub host: String,
    pub port: i32,
    pub vhost: String,
    pub username: String,
    pub password: String,
    pub heartbeat: i32,
    pub connection_timeout: i32,
    pub retry: u64,
}

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct BindingProperties {
    pub queue: String,
    pub exchange: String,
    pub routing_key: String,
}

#[serde(default)]
#[derive(Deserialize, Clone, Debug)]
pub struct DeclareProperties {
    pub queue: bool,
    pub exchange: bool,
    pub binding: bool,
}

impl Default for JSONConfiguration {
    fn default() -> JSONConfiguration {
        JSONConfiguration {
            connection: ConnectionProperties::default(),
            binding: BindingProperties::default(),
            declare: DeclareProperties::default(),
        }
    }
}

impl Default for ConnectionProperties {
    fn default() -> ConnectionProperties {
        ConnectionProperties {
            host: "127.0.0.1".to_string(),
            port: 5672,
            vhost: "/".to_string(),
            heartbeat: 10,
            connection_timeout: 1000,
            username: "guest".to_string(),
            password: "guest".to_string(),
            retry: 6,
        }
    }
}

impl Default for BindingProperties {
    fn default() -> BindingProperties {
        BindingProperties {
            queue: "myQueue".to_string(),
            exchange: "myExchange".to_string(),
            routing_key: "myKey".to_string(),
        }
    }
}

impl Default for DeclareProperties {
    fn default() -> DeclareProperties {
        DeclareProperties {
            queue: true,
            exchange: true,
            binding: true,
        }
    }
}
