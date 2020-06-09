use crate::configuration::config_model::BindingProperties;
use crate::configuration::config_model::ConnectionProperties;
use crate::configuration::config_model::DeclareProperties;
use crate::consumer::connection_manager::build_url;
use crate::consumer::connection_manager::create_channel;
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

///  # Executes configuration defined in the configuration
///  * Creates a channel
///  * Declares the Queue
///  * Declares the Exchange
///  * Declares the Binding between the Exchange and the Queue
///
pub async fn setup_consumer(
    connection: ConnectionProperties,
    bind: BindingProperties,
    declare: DeclareProperties,
) -> SetupModel {
    let channel = create_channel(build_url(connection.clone()).as_str(), connection.retry)
        .await
        .expect("channel to be created");
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
    }
    let mut exchange = None;
    if declare.exchange {
        exchange = Some(
            create_exchange(
                &bind.exchange,
                channel.clone(),
                ExchangeDeclareOptions {
                    passive: bind.exchange_declaration_options.passive,
                    durable: bind.exchange_declaration_options.durable,
                    auto_delete: bind.exchange_declaration_options.auto_delete,
                    internal: false,
                    nowait: false,
                },
            )
            .await,
        );
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
    }

    return SetupModel {
        channel,
        queue,
        exchange,
        binding,
    };
}

/// # Creates an exchange
/// * Returns an exchange
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

/// # Binds the exchange with a queue
/// * Returns the binding
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