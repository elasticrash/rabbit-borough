use futures_executor::LocalPool;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, ExchangeKind};

fn main() {
    std::env::set_var("RUST_LOG", "info");

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    LocalPool::new().run_until(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        println!("CONNECTED");

        //receive channel
        let channel = conn.create_channel().await.unwrap();

        println!("[{}] state: {:?}", line!(), conn.status().state());

        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();
        println!("[{}] state: {:?}", line!(), conn.status().state());

        let exchange = channel
            .exchange_declare(
                "main.x",
                ExchangeKind::Fanout,
                ExchangeDeclareOptions {
                    passive: false,
                    durable: false,
                    auto_delete: true,
                    internal: false,
                    nowait: true,
                },
                FieldTable::default(),
            )
            .await
            .unwrap();

        println!("[{}] state: {:?}", line!(), conn.status().state());
        println!("declared exchange {:?}", exchange);

        let bind = channel
            .exchange_bind(
                "hello",
                "main.x",
                "rust",
                ExchangeBindOptions { nowait: true },
                FieldTable::default(),
            )
            .await
            .unwrap();

        println!("[{}] state: {:?}", line!(), conn.status().state());
        println!("bind {:?}", bind);

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
        println!("[{}] state: {:?}", line!(), conn.status().state());

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
