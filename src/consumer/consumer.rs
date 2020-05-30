use crate::consumer::handler_message_result::action_result;
use crate::consumer::handler_message_result::HandleMessageResult;
use lapin::message::Delivery;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;
use lapin::Channel;
use lapin::Consumer;

pub async fn create_consumer(queue_name: &str, channel: &Channel) -> Consumer {
    let consumer = channel
        .basic_consume(
            queue_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("basic_consume");
    return consumer;
}

pub async fn consume(consumer: Consumer, handler: &dyn Fn(&Delivery) -> HandleMessageResult) {
    for delivery in consumer {
        if let Ok((channel, delivery)) = delivery {
            action_result(handler(&delivery), &channel, delivery.delivery_tag).await;
        }
    }
}
