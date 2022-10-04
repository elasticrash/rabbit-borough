use crate::configuration::config_model::JSONConfiguration;
use crate::consumer::handle_message_result::action_result;
use crate::consumer::handle_message_result::HandleMessageResult;
use crate::consumer::setup::setup_consumer;
use futures_lite::StreamExt;
use lapin::message::Delivery;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;
use lapin::Channel;
use lapin::Consumer;

pub type DeliveredMessage = Delivery;

/// # Create a consumer
/// Returns consumer
pub async fn create_consumer(queue_name: &str, channel: &Channel) -> Consumer {
    channel
        .basic_consume(
            queue_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("basic_consume")
}

/// This is the function that the consuming is happening
/// There are two (infinite) loops here, a. one around the consumer, which
/// while the connection is healthy keeps dequeuing messages
/// b. one that once the connection dies, restarts the whole process of getting a channel and
/// setting up the consumer.
///
/// this function also allows you to pass an argument of type T into the handler
pub async fn consume_with_option<T>(
    config: &JSONConfiguration,
    handler: &dyn Fn(&Delivery, Option<&T>) -> HandleMessageResult,
    args: Option<&T>,
) -> HandleMessageResult {
    loop {
        let model = setup_consumer(
            config.connection.clone(),
            config.binding.clone(),
            config.declare.clone(),
        )
        .await;

        println!("[{}] - {:?}", line!(), model.channel.status().state());

        let mut consumer = create_consumer(&config.binding.queue, &model.channel).await;

        println!("[{}] - {:?}", line!(), consumer.tag());

        while let Some(res_delivery) = consumer.next().await {
            let delivery = res_delivery.unwrap();
            action_result(
                handler(&delivery, args),
                &model.channel,
                delivery.delivery_tag,
            )
            .await;
        }
    }
}

/// This is the function that the consuming is happening
/// There are two (infinite) loops here, a. one around the consumer, which
/// while the connection is healthy keeps dequeuing messages
/// b. one that once the connection dies, restarts the whole process of getting a channel and
/// setting up the consumer
pub async fn consume(
    config: &JSONConfiguration,
    handler: &dyn Fn(&Delivery) -> HandleMessageResult,
) -> HandleMessageResult {
    loop {
        let model = setup_consumer(
            config.connection.clone(),
            config.binding.clone(),
            config.declare.clone(),
        )
        .await;

        println!("[{}] - {:?}", line!(), model.channel.status().state());

        let mut consumer = create_consumer(&config.binding.queue, &model.channel).await;

        println!("[{}] - {:?}", line!(), consumer.tag());

        while let Some(res_delivery) = consumer.next().await {
            let delivery = res_delivery.unwrap();
            action_result(handler(&delivery), &model.channel, delivery.delivery_tag).await;
        }
    }
}
