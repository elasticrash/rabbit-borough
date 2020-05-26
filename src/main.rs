mod configuration;
mod consumer;
use crate::consumer::consumer_configuration::ConsumerConfiguration;
use configuration::config_model::JSONConfiguration;
use futures_executor::LocalPool;
use lapin::message::Delivery;
use lapin::options::BasicAckOptions;
use lapin::options::BasicConsumeOptions;
use lapin::options::BasicNackOptions;
use lapin::types::FieldTable;
use lapin::types::LongLongUInt;
use lapin::Channel;

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
        connection_retry: &6
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
                handler(&model.channel, delivery).await;
            }
        }
    })
}

pub enum HandleMessageResult {
    Ack,
    NackNoRequeue,
    NackWithRequeue,
}

// function to handle the message
async fn handler(channel: &Channel, delivery: Delivery) {
    action_result(HandleMessageResult::Ack, &channel, delivery.delivery_tag).await;
}

// output of that handler
async fn action_result(result: HandleMessageResult, channel: &Channel, tag: LongLongUInt) {
    match result {
        HandleMessageResult::Ack => {
            channel
                .basic_ack(tag, BasicAckOptions { multiple: false })
                .await
                .expect("basic_ack");
        }
        HandleMessageResult::NackNoRequeue => {
            channel
                .basic_nack(
                    tag,
                    BasicNackOptions {
                        multiple: false,
                        requeue: false,
                    },
                )
                .await
                .expect("basic_ack");
        }
        HandleMessageResult::NackWithRequeue => {
            channel
                .basic_nack(
                    tag,
                    BasicNackOptions {
                        multiple: false,
                        requeue: true,
                    },
                )
                .await
                .expect("basic_ack");
        }
    }
}
