use crate::configuration::config_model::BindingProperties;
use crate::configuration::config_model::ConnectionProperties;
use crate::configuration::config_model::DeclareProperties;
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
    pub queue: Option<Result<Queue, Error>>,
    pub exchange: Option<Result<(), Error>>,
    pub binding: Option<Result<(), Error>>,
}

pub async fn setup_consumer(
    connection: ConnectionProperties,
    bind: BindingProperties,
    declare: DeclareProperties,
) -> SetupModel {
    let channel = get_channel(connection).await;
    let mut queue = None;
    if declare.queue {
        queue = Some(
            channel
                .queue_declare(
                    &bind.queue,
                    QueueDeclareOptions::default(),
                    FieldTable::default(),
                )
                .await,
        );
        println!("[{}] queue setup completed: {:?}", line!(), queue);
    }
    let mut exchange = None;
    if declare.exchange {
        exchange = Some(
            create_exchange(
                &bind.exchange,
                channel.clone(),
                ExchangeDeclareOptions {
                    passive: false,
                    durable: true,
                    auto_delete: true,
                    internal: false,
                    nowait: false,
                },
            )
            .await,
        );
        println!("[{}] exchange setup completed: {:?}", line!(), exchange);
    }

    let mut binding = None;
    if declare.binding {
        binding = Some(
            create_exchange_queue_binding(
                channel.clone(),
                &bind.queue,
                &bind.exchange,
                &bind.routing_key,
            )
            .await,
        );
        println!(
            "[{}] queue/exchange binding completed: {:?}",
            line!(),
            binding
        );
    }

    return SetupModel {
        channel,
        queue,
        exchange,
        binding,
    };
}

pub async fn get_channel(connection: ConnectionProperties) -> Channel {
    return create_channel(build_url(connection.clone()).as_str(), connection.retry).await;
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
