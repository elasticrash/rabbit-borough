use crate::configuration::config_model::ConnectionProperties as LocalProperties;
use futures::future::{BoxFuture, FutureExt};
use lapin::Channel;
use lapin::{Connection, ConnectionProperties};
use std::{thread, time};

#[derive(Debug)]
pub struct GetConnectionError {
    pub why: ConnectionState,
    pub last_reason: lapin::Error,
}

#[derive(Debug)]
pub enum ConnectionState {
    MaximumConnectionRetriesReached,
    Unknown,
}

/// Returns a Connection, retries x times
/// This is an async resursive function
pub fn get_connection<'a>(
    addr: &'a str,
    retry: u64,
    total_retries: u64,
) -> BoxFuture<'a, Result<Connection, GetConnectionError>> {
    return Box::pin(
        async move {
            let con_promise = Connection::connect(
                &addr,
                ConnectionProperties::default().with_default_executor(8),
            );
            let conn_res = con_promise.await;
            let connection = match conn_res {
                Ok(c) => Ok(c),
                Err(why) => {
                    println!("[{}] - {:?}", line!(), why);
                    if retry > total_retries {
                        GetConnectionError {
                            why: ConnectionState::MaximumConnectionRetriesReached,
                            last_reason: why,
                        };
                    }
                    let hibernate = time::Duration::from_millis(retry * 1000);
                    thread::sleep(hibernate);
                    let c = get_connection(addr, retry + 1, total_retries);
                    c.await
                }
            };
            connection
        }
        .boxed(),
    );
}

/// # builds URL
/// although heartbeat and connection_timeout are optional
/// parameters, they are really useful and allow you to fail
/// easier and more precisely. So they are used by default.
pub fn build_url(config: LocalProperties) -> String {
    let url = format!(
        "amqp://{}:{}@{}:{}/{}?hearthbeat={}&connection_timeout={}",
        config.username,
        config.password,
        config.host,
        config.port,
        config.vhost,
        config.heartbeat,
        config.connection_timeout
    );

    return url;
}

/// # Creates a channel
/// * Gets a valid connection
/// * Returns a channel
pub async fn create_channel<'a>(
    addr: &'a str,
    total_retries: u64,
) -> Result<Channel, GetConnectionError> {
    let conn = get_connection(&addr, 0, total_retries).await?;

    return match conn.create_channel().await {
        Ok(ch) => Ok(ch),
        Err(why) => panic!("{}", why),
    };
}

#[cfg(test)]
mod tests {
    use crate::configuration::config_model::*;
    use crate::consumer::connection_manager::build_url;

    #[test]
    fn amqp_url_generated_succesfully() {
        let url = build_url(ConnectionProperties::default());
        assert_eq!(
            "amqp://guest:guest@127.0.0.1:5672//?hearthbeat=10&connection_timeout=1000",
            url
        );
    }
}
