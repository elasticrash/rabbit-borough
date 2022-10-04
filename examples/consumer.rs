extern crate rabbit_borough;

use futures::executor::LocalPool;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::consumer::actions::consume;
use rabbit_borough::consumer::actions::DeliveredMessage;
use rabbit_borough::consumer::handle_message_result::HandleMessageResult;

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        consume(&config, &handler).await;
    })
}

/// function to handle the message
fn handler(_delivery: &DeliveredMessage) -> HandleMessageResult {
    // CONSUMER LOGIC
    println!("[{}] - {:?}", line!(), std::str::from_utf8(&_delivery.data));
    return HandleMessageResult::Ack;
}
