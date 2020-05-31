mod configuration;
mod consumer;

use crate::consumer::consumer::consume;
use crate::consumer::consumer::create_consumer;
use crate::consumer::handler_message_result::HandleMessageResult;
use configuration::config_model::JSONConfiguration;
use futures_executor::LocalPool;
use lapin::message::Delivery;

fn main() {
    let config: JSONConfiguration = match configuration::reader::read("./config.json") {
        Ok(data) => data,
        Err(why) => panic!("Error {:?}", why),
    };

    LocalPool::new().run_until(async {
        let model = consumer::setup::setup_consumer(config.connection.clone(), config.binding.clone()).await;

        println!(
            "[{}] channel status: {:?}",
            line!(),
            model.channel.status().state()
        );

        println!("[{}] queue status: {:?}", line!(), model.queue);
        println!("[{}] exchange status: {:?}", line!(), model.exchange);
        println!("[{}] bind status: {:?}", line!(), model.binding);
        let consumer = create_consumer(&config.binding.queue, &model.channel).await;
        consume(consumer, &handler).await;
    })
}

/// function to handle the message
fn handler(_delivery: &Delivery) -> HandleMessageResult {
    // CONSUMER LOGIC
    println!("[{}] - {:?}", line!(), std::str::from_utf8(&_delivery.data));
    return HandleMessageResult::Ack;
}
