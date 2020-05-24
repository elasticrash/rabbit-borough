mod configuration;
mod consumer;
use crate::consumer::consumer_configuration::ConsumerConfiguration;
use configuration::config_model::JSONConfiguration;
use futures_executor::LocalPool;
use lapin::options::BasicAckOptions;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;

fn main() {
    let config: JSONConfiguration = match configuration::reader::read("./config.json") {
        Ok(data) => data,
        Err(why) => panic!("Error {:?}", why),
    };

    let setup_config = ConsumerConfiguration {
        host: &config.host,
        port: &config.port,
        vhost: &config.vhost,
        username: &config.username,
        password: &config.password,
        heartbeat: &config.heartbeat,
        connection_timeout: &config.connection_timeout,
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
                "hello",
                "rust",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("basic_consume");

        for delivery in consumer {
            println!("received message: {:?}", delivery);
            if let Ok(delivery) = delivery {
                model
                    .channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
        }
    })
}
