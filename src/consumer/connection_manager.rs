use crate::configuration::config_model::ConnectionProperties as LocalProperties;
use async_io::Timer;
use lapin::{Channel, Connection, ConnectionProperties, Error};
use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use std::time::Duration;

#[derive(Debug)]
pub struct GenericError<T> {
    pub why: ErrorType,
    pub last_reason: T,
}

#[derive(Debug)]
pub enum ErrorType {
    MaximumConnectionRetriesReached,
    CannotCreateChannel,
    Unknown,
}

/// Returns a Connection.
/// The first attempt is made immediately; on failure it waits `attempt` seconds
pub async fn get_connection(
    addr: &str,
    retry: u64,
    total_retries: u64,
) -> Result<Connection, GenericError<Error>> {
    let mut attempt = retry;
    loop {
        match Connection::connect(addr, ConnectionProperties::default()).await {
            Ok(connection) => return Ok(connection),
            Err(why) => {
                println!("[{}] - {:?}", line!(), why);
                if attempt >= total_retries {
                    return Err(GenericError {
                        why: ErrorType::MaximumConnectionRetriesReached,
                        last_reason: why,
                    });
                }
                attempt += 1;
                // async sleep: never block the executor thread
                Timer::after(Duration::from_secs(attempt)).await;
            }
        }
    }
}

/// # builds URL
/// although heartbeat and connection_timeout are optional
/// parameters, they are really useful and allow you to fail
/// easier and more precisely. So they are used by default.
pub fn build_url(config: LocalProperties) -> String {
    format!(
        "amqp://{}:{}@{}:{}/{}?heartbeat={}&connection_timeout={}",
        utf8_percent_encode(&config.username, NON_ALPHANUMERIC),
        utf8_percent_encode(&config.password, NON_ALPHANUMERIC),
        config.host,
        config.port,
        utf8_percent_encode(&config.vhost, NON_ALPHANUMERIC),
        config.heartbeat,
        config.connection_timeout
    )
}

/// # Creates a connection and a channel on it
/// * Gets a valid connection (retrying up to `total_retries` times)
/// * Returns the connection together with a channel
///
/// The caller owns the connection and should `close` it when done,
/// otherwise it stays open in the background.
pub async fn create_connection_and_channel(
    addr: &str,
    total_retries: u64,
) -> Result<(Connection, Channel), GenericError<Error>> {
    let conn = get_connection(addr, 0, total_retries).await?;

    match conn.create_channel().await {
        Ok(ch) => Ok((conn, ch)),
        Err(why) => {
            let _ = conn.close(0, "cannot create channel".into()).await;
            Err(GenericError {
                why: ErrorType::CannotCreateChannel,
                last_reason: why,
            })
        }
    }
}

/// # Creates a channel
/// * Gets a valid connection
/// * Returns a channel
///
/// The underlying connection stays open for as long as the channel is in use.
/// Prefer [`create_connection_and_channel`] if you need to close it afterwards.
pub async fn create_channel(
    addr: &str,
    total_retries: u64,
) -> Result<Channel, GenericError<Error>> {
    create_connection_and_channel(addr, total_retries)
        .await
        .map(|(_, channel)| channel)
}

#[cfg(test)]
mod tests {
    use crate::configuration::config_model::*;
    use crate::consumer::connection_manager::build_url;

    #[test]
    fn amqp_url_generated_succesfully() {
        let url = build_url(ConnectionProperties::default());
        assert_eq!(
            "amqp://guest:guest@127.0.0.1:5672/%2F?heartbeat=10&connection_timeout=1000",
            url
        );
    }

    #[test]
    fn amqp_url_escapes_credentials_and_vhost() {
        let url = build_url(ConnectionProperties {
            username: "us:er".to_string(),
            password: "p@ss/word".to_string(),
            vhost: "my vhost".to_string(),
            ..ConnectionProperties::default()
        });
        assert_eq!(
            "amqp://us%3Aer:p%40ss%2Fword@127.0.0.1:5672/my%20vhost?heartbeat=10&connection_timeout=1000",
            url
        );
    }
}
