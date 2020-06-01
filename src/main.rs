mod configuration;
mod consumer;

use crate::consumer::consumer::consume;
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
        consume(&config, &handler).await;
    })
}

/// function to handle the message
fn handler(_delivery: &Delivery) -> HandleMessageResult {
    // CONSUMER LOGIC
    println!("[{}] - {:?}", line!(), std::str::from_utf8(&_delivery.data));
    return HandleMessageResult::Ack;
}
