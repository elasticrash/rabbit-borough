use futures::future::{BoxFuture, FutureExt};
use futures_executor::LocalPool;
use lapin::types::FieldTable;
use lapin::Channel;
use lapin::{options::*, Connection, ConnectionProperties, ExchangeKind};
use std::{thread, time};

fn main() {
    std::env::set_var("RUST_LOG", "info");
    let addr: &'static str = "amqp://127.0.0.1:5672/%2f";
    LocalPool::new().run_until(async {
        let channel = get_channel(addr).await;

        println!(
            "[{}] channel status: {:?}",
            line!(),
            channel.status().state()
        );

        let exchange = channel
            .exchange_declare(
                "main.x",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .wait();
        println!("[{}] exchange status: {:?}", line!(), exchange);

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await;

        println!("[{}] queue status: {:?}", line!(), queue);

        let bind = channel
            .queue_bind(
                "hello",
                "main.x",
                "rust",
                QueueBindOptions { nowait: false },
                FieldTable::default(),
            )
            .wait();

        println!("[{}] bind status: {:?}", line!(), bind);

        println!("will consume");
        let consumer = channel
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        for delivery in consumer {
            println!("received message: {:?}", delivery);
            if let Ok(delivery) = delivery {
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    })
}

async fn get_channel(addr: &'static str) -> Channel {
    let conn = get_connection(&addr, 0).await;
    println!(
        "[{}] connection state: {:?}",
        line!(),
        conn.status().state()
    );

    return match conn.create_channel().await {
        Ok(ch) => ch,
        Err(why) => panic!("{}", why),
    };
}

/// Returns a Connection, retries x times
fn get_connection(addr: &'static str, retry: u64) -> BoxFuture<'static, Connection> {
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
                    let hibernate = time::Duration::from_millis(retry * 100);
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
