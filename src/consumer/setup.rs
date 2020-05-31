use crate::configuration::config_model::BindingProperties;
use crate::configuration::config_model::ConnectionProperties;
use crate::consumer::connection_manager;
use lapin::options::ExchangeDeclareOptions;
use lapin::options::*;
use lapin::types::FieldTable;
use lapin::Channel;
use lapin::Error;
use lapin::ExchangeKind;
use lapin::Queue;

#[derive(Clone)]
pub struct SetupModel {
    pub channel: Channel,
    pub queue: Result<Queue, Error>,
    pub exchange: Result<(), Error>,
    pub binding: Result<(), Error>,
}

pub async fn setup_consumer(
    connection: ConnectionProperties,
    binding: BindingProperties,
) -> SetupModel {
    let channel = create_channel(build_url(connection.clone()).as_str(), connection.retry).await;
    let queue = channel
        .queue_declare(
            &binding.queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await;
    let exchange = create_exchange(
        &binding.exchange,
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

    let binding = create_exchange_queue_binding(
        channel.clone(),
        &binding.queue,
        &binding.exchange,
        &binding.routing_key,
    )
    .await;

    return SetupModel {
        channel,
        queue,
        exchange,
        binding,
    };
}

/// build URL
fn build_url(config: ConnectionProperties) -> String {
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

/// create a channel
async fn create_channel<'a>(addr: &'a str, total_retries: u64) -> Channel {
    let conn = connection_manager::get_connection(&addr, 0, total_retries).await;
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
async fn create_exchange(
    exchange: &str,
    channel: Channel,
    options: ExchangeDeclareOptions,
) -> Result<(), Error> {
    let exchange = channel
        .exchange_declare(
            exchange,
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
