use crate::configuration::config_model::BindingProperties;
use crate::configuration::config_model::ConnectionProperties;
use crate::configuration::config_model::DeclareProperties;
use crate::consumer::connection_manager::GenericError;
use crate::consumer::connection_manager::build_url;
use crate::consumer::connection_manager::create_connection_and_channel;
use lapin::Channel;
use lapin::Connection;
use lapin::Error;
use lapin::ExchangeKind;
use lapin::Queue;
use lapin::options::*;
use lapin::types::FieldTable;

pub struct SetupModel {
    pub connection: Connection,
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
/// Returns an error if no connection could be established after
/// exhausting all configured retries.
pub async fn setup_consumer(
    connection: ConnectionProperties,
    bind: BindingProperties,
    declare: DeclareProperties,
) -> Result<SetupModel, GenericError<Error>> {
    let (conn, channel) =
        create_connection_and_channel(build_url(connection.clone()).as_str(), connection.retry)
            .await?;

    let mut queue = None;
    if declare.queue {
        queue = Some(
            channel
                .queue_declare(
                    bind.queue.as_str().into(),
                    QueueDeclareOptions {
                        passive: bind.queue_declaration_options.passive,
                        durable: bind.queue_declaration_options.durable,
                        exclusive: bind.queue_declaration_options.exclusive,
                        auto_delete: bind.queue_declaration_options.auto_delete,
                        nowait: false,
                    },
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
                &channel,
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
            create_exchange_queue_binding(&channel, &bind.queue, &bind.exchange, &bind.routing_key)
                .await,
        );
    }

    Ok(SetupModel {
        connection: conn,
        channel,
        queue,
        exchange,
        binding,
    })
}

/// # Creates an exchange
/// * Returns an exchange
async fn create_exchange(
    exchange: &str,
    channel: &Channel,
    options: ExchangeDeclareOptions,
) -> Result<(), Error> {
    channel
        .exchange_declare(
            exchange.into(),
            ExchangeKind::Fanout,
            options,
            FieldTable::default(),
        )
        .await
}

/// # Binds the exchange with a queue
/// * Returns the binding
async fn create_exchange_queue_binding(
    channel: &Channel,
    queue: &str,
    exchange: &str,
    routing_key: &str,
) -> Result<(), Error> {
    channel
        .queue_bind(
            queue.into(),
            exchange.into(),
            routing_key.into(),
            QueueBindOptions { nowait: false },
            FieldTable::default(),
        )
        .await
}
