use futures::future::{BoxFuture, FutureExt};
use lapin::{Connection, ConnectionProperties};
use std::{thread, time};

/// Returns a Connection, retries x times
pub fn get_connection(addr: &'static str, retry: u64) -> BoxFuture<'static, Connection> {
    return Box::pin(
        async move {
            let con_promise = Connection::connect(&addr, ConnectionProperties::default());
            let conn_res = con_promise.await;
            let connection = match conn_res {
                Ok(c) => c,
                Err(why) => {
                    println!("{}", why);
                    if retry > 5 {
                        panic!("maximum retries reached");
                    }
                    let hibernate = time::Duration::from_millis(retry * 1000);
                    thread::sleep(hibernate);
                    let c = get_connection(addr, retry + 1);
                    c.await
                }
            };
            connection
        }
        .boxed(),
    );
}
