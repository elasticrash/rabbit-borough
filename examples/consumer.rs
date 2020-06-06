extern crate rabbit_borough;

use futures_executor::LocalPool;
use lapin::message::Delivery;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::consumer::consumer::consume;
use rabbit_borough::consumer::handle_message_result::HandleMessageResult;

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

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
