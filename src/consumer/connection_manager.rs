use futures::future::{BoxFuture, FutureExt};
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
