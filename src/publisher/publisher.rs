use crate::configuration::config_model::ConnectionProperties;
use crate::consumer::connection_manager::build_url;
use crate::consumer::connection_manager::create_channel;
use amq_protocol_types::ShortString;
use lapin::options::BasicPublishOptions;
use lapin::publisher_confirm::PublisherConfirm;
use lapin::BasicProperties;

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
    let channel = create_channel(build_url(connection.clone()).as_str(), connection.retry)
        .await
        .expect("channel to be created");

    let outcome = match channel
        .basic_publish(
            exchange_name,
            routing_key,
            BasicPublishOptions::default(),
            message.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
    {
        Ok(result) => Ok(result),
        Err(why) => Err(PublishError { why }),
    };

    let _closed = channel.close(0, "shutting down").await;

    return outcome;
}

// Sets up publisher and sends message
pub async fn publish_with_type(
    message: String,
    exchange_name: &str,
    routing_key: &str,
    connection: ConnectionProperties,
    message_type: &str,
) -> Result<PublisherConfirm, PublishError> {
    let channel = create_channel(build_url(connection.clone()).as_str(), connection.retry)
        .await
        .expect("channel to be created");

    let base_properties = BasicProperties::default();
    let final_properties = base_properties.with_content_type(ShortString::from(message_type));

    let outcome = match channel
        .basic_publish(
            exchange_name,
            routing_key,
            BasicPublishOptions::default(),
            message.as_bytes().to_vec(),
            final_properties,
        )
        .await
    {
        Ok(result) => Ok(result),
        Err(why) => Err(PublishError { why }),
    };

    let _closed = channel.close(0, "shutting down").await;

    return outcome;
}
