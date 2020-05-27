mod configuration;
mod consumer;
use crate::consumer::consumer_configuration::ConsumerConfiguration;
use crate::consumer::handler_message_result::{action_result, HandleMessageResult};
use configuration::config_model::JSONConfiguration;
use futures_executor::LocalPool;
use lapin::message::Delivery;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;

fn main() {
    let config: JSONConfiguration = match configuration::reader::read("./config.json") {
        Ok(data) => data,
        Err(why) => panic!("Error {:?}", why),
    };

    let setup_config = ConsumerConfiguration {
        host: &config.connection.host,
        port: &config.connection.port,
        vhost: &config.connection.vhost,
        username: &config.connection.username,
        password: &config.connection.password,
        heartbeat: &config.connection.heartbeat,
        connection_timeout: &config.connection.connection_timeout,
        queue: &config.binding.queue,
        exchange: &config.binding.exchange,
        routing_key: &config.binding.routing_key,
        connection_retry: &config.connection.retry,
    };

    LocalPool::new().run_until(async {
        let model = consumer::setup::setup_consumer(setup_config).await;

        println!(
            "[{}] channel status: {:?}",
            line!(),
            model.channel.status().state()
        );

        println!("[{}] queue status: {:?}", line!(), model.queue);
        println!("[{}] exchange status: {:?}", line!(), model.exchange);
        println!("[{}] bind status: {:?}", line!(), model.binding);

        let consumer = model
            .channel
            .basic_consume(
                &config.binding.queue,
                "",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        for delivery in consumer {
            println!("received message: {:?}", delivery);
            if let Ok(delivery) = delivery {
                let tag = delivery.delivery_tag.clone();
                let outcome = handler(delivery);
                action_result(outcome, &model.channel, tag).await;
            }
        }
    })
}

/// function to handle the message
fn handler(_delivery: Delivery) -> HandleMessageResult {
    // CONSUMER LOGIC
    return HandleMessageResult::Ack;
}
