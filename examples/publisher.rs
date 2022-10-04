extern crate rabbit_borough;

use futures::executor::LocalPool;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::publisher::actions::publish;
use std::{thread, time};

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        loop {
            let outcome = publish(
                "test".to_string(),
                &config.binding.exchange,
                &config.binding.routing_key,
                config.connection.clone(),
            )
            .await;

            println!("[{}] - {:?}", line!(), outcome);

            let delay = time::Duration::from_millis(500);
            thread::sleep(delay);
        }
    });
}
