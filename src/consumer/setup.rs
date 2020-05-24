use crate::consumer::connection_manager;
use lapin::options::ExchangeDeclareOptions;
use lapin::options::*;
use lapin::types::FieldTable;
use lapin::Channel;
use lapin::Error;
use lapin::ExchangeKind;
use lapin::Queue;

pub struct SetupModel {
    pub channel: Channel,
    pub queue: Result<Queue, Error>,
    pub exchange: Result<(), Error>,
    pub binding: Result<(), Error>,
}

pub async fn setup_consumer(addr: &'static str) -> SetupModel {
    let channel = create_channel(addr).await;
    let queue = channel
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await;
    let exchange = create_exchange(
        channel.clone(),
        ExchangeDeclareOptions {
            passive: false,
            durable: true,
            auto_delete: true,
            internal: false,
            nowait: false,
        },
    )
    .await;

    let binding = create_exchange_queue_binding(channel.clone(), "hello", "main.x", "rust").await;

    return SetupModel {
        channel,
        queue,
        exchange,
        binding,
    };
}

/// create a channel
async fn create_channel(addr: &'static str) -> Channel {
    let conn = connection_manager::get_connection(&addr, 0).await;
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

/// create an exchange
async fn create_exchange(channel: Channel, options: ExchangeDeclareOptions) -> Result<(), Error> {
    let exchange = channel
        .exchange_declare(
            "main.x",
            ExchangeKind::Fanout,
            options,
            FieldTable::default(),
        )
        .wait();
    return exchange;
}

/// bind your exchange with a queue
async fn create_exchange_queue_binding(
    channel: Channel,
    queue: &str,
    exchange: &str,
    routing_key: &str,
) -> Result<(), Error> {
    let bind = channel
        .queue_bind(
            queue,
            exchange,
            routing_key,
            QueueBindOptions { nowait: false },
            FieldTable::default(),
        )
        .wait();

    return bind;
}
