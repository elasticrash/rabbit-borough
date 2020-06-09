extern crate rabbit_borough;

use futures_executor::LocalPool;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::publisher::publisher::publish;
use std::{thread, time};

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        loop {
            publish(
                "test".to_string(),
                &config.binding.exchange,
                &config.binding.routing_key,
                config.connection.clone(),
            )
            .await;

            let delay = time::Duration::from_millis(500);
            thread::sleep(delay);
        }
    });
}
