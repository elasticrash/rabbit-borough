use crate::configuration::config_model::ConnectionProperties;
use crate::consumer::connection_manager::build_url;
use crate::consumer::connection_manager::create_connection_and_channel;
use lapin::BasicProperties;
use lapin::PublisherConfirm;
use lapin::options::BasicPublishOptions;
use lapin::types::ShortString;

#[derive(Debug)]
pub struct PublishError {
    pub why: lapin::Error,
}

/// Sets up publisher and sends message
pub async fn publish(
    message: String,
    exchange_name: &str,
    routing_key: &str,
    connection: ConnectionProperties,
) -> Result<PublisherConfirm, PublishError> {
    publish_with_properties(
        message,
        exchange_name,
        routing_key,
        connection,
        BasicProperties::default(),
    )
    .await
}

/// Sets up publisher and sends message with the given content type
pub async fn publish_with_type(
    message: String,
    exchange_name: &str,
    routing_key: &str,
    connection: ConnectionProperties,
    message_type: &str,
) -> Result<PublisherConfirm, PublishError> {
    publish_with_properties(
        message,
        exchange_name,
        routing_key,
        connection,
        BasicProperties::default().with_content_type(ShortString::from(message_type)),
    )
    .await
}

/// Opens a one-off connection, publishes, then closes the connection.
async fn publish_with_properties(
    message: String,
    exchange_name: &str,
    routing_key: &str,
    connection: ConnectionProperties,
    properties: BasicProperties,
) -> Result<PublisherConfirm, PublishError> {
    let (conn, channel) =
        create_connection_and_channel(build_url(connection.clone()).as_str(), connection.retry)
            .await
            .map_err(|e| PublishError { why: e.last_reason })?;

    let outcome = channel
        .basic_publish(
            exchange_name.into(),
            routing_key.into(),
            BasicPublishOptions::default(),
            message.as_bytes(),
            properties,
        )
        .await
        .map_err(|why| PublishError { why });

    // closing the connection also closes the channel, and releases the socket
    let _ = conn.close(0, "shutting down".into()).await;

    outcome
}
