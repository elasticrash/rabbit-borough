use crate::configuration::config_model::JSONConfiguration;
use crate::consumer::handler_message_result::action_result;
use crate::consumer::handler_message_result::HandleMessageResult;
use crate::consumer::setup::get_channel;
use crate::consumer::setup::setup_consumer;
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

pub async fn consume(
    config: &JSONConfiguration,
    handler: &dyn Fn(&Delivery) -> HandleMessageResult,
) {
    loop {
        let model = setup_consumer(
            config.connection.clone(),
            config.binding.clone(),
            config.declare.clone(),
        )
        .await;
        println!(
            "[{}] channel status: {:?}",
            line!(),
            model.channel.status().state()
        );
        let consumer = create_consumer(&config.binding.queue, &model.channel).await;
        for message in consumer {
            match message {
                Ok((channel, delivery)) => {
                    action_result(handler(&delivery), &channel, delivery.delivery_tag).await;
                }
                Err(why) => {
                    println!("[{}] channel status: {:?}", line!(), why);
                }
            };
        }
    }
}
