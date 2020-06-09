use crate::configuration::config_model::ConnectionProperties;
use crate::consumer::connection_manager::build_url;
use crate::consumer::connection_manager::create_channel;
use lapin::options::BasicPublishOptions;
use lapin::publisher_confirm::PublisherConfirm;
use lapin::BasicProperties;

pub async fn publish(
    message: String,
    exchange_name: &str,
    routing_key: &str,
    connection: ConnectionProperties,
) -> PublisherConfirm {
    let channel = create_channel(build_url(connection.clone()).as_str(), connection.retry)
        .await
        .expect("channel to be created");
    let output = match channel
        .basic_publish(
            exchange_name,
            routing_key,
            BasicPublishOptions::default(),
            message.as_bytes().to_vec(),
            BasicProperties::default(),
        )
        .await
    {
        Ok(result) => result,
        Err(_why) => panic!("failed to publish"),
    };

    let closed = channel.close(0, "shutting down").await;
    println!("{:?}", output);
    println!("{:?}", closed);

    return output;
}
