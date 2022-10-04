extern crate rabbit_borough;

use futures::executor::LocalPool;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::consumer::actions::consume_with_option;
use rabbit_borough::consumer::actions::DeliveredMessage;
use rabbit_borough::consumer::handle_message_result::HandleMessageResult;

#[derive(Debug)]
struct RandomObject {
    property_a: u32,
    property_b: String,
}

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        consume_with_option(
            &config,
            &handler,
            Some(&RandomObject {
                property_a: 22,
                property_b: "test".to_string(),
            }),
        )
        .await;
    })
}

/// function to handle the message
fn handler(_delivery: &DeliveredMessage, _ref: Option<&RandomObject>) -> HandleMessageResult {
    // CONSUMER LOGIC
    println!("[{}] - {:?}", line!(), std::str::from_utf8(&_delivery.data));
    println!("[{}] - {:?}", line!(), _ref);

    return HandleMessageResult::Ack;
}
