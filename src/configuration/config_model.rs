use serde::Deserialize;

/// Top level configuration class
#[derive(Default, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct JSONConfiguration {
    pub connection: ConnectionProperties,
    pub binding: BindingProperties,
    pub declare: DeclareProperties,
}

/// All the properties required for creating a connection
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
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

/// Configuration for binding an Queue to an Exchange
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct BindingProperties {
    pub queue: String,
    pub exchange: String,
    pub routing_key: String,
    pub exchange_declaration_options: ExchangeOptions,
}

/// Configuration on whether some setup should be deemed unnecessary
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct DeclareProperties {
    pub queue: bool,
    pub exchange: bool,
    pub binding: bool,
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(default)]
pub struct ExchangeOptions {
    pub passive: bool,
    pub durable: bool,
    pub auto_delete: bool,
}

impl Default for ExchangeOptions {
    fn default() -> ExchangeOptions {
        ExchangeOptions {
            passive: false,
            durable: true,
            auto_delete: true,
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
            exchange_declaration_options: ExchangeOptions::default(),
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
