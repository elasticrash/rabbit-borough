use futures::executor::LocalPool;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::publisher::actions::publish_with_type;
use std::time::Duration;

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        loop {
            let outcome = publish_with_type(
                "test".to_string(),
                &config.binding.exchange,
                &config.binding.routing_key,
                config.connection.clone(),
                "ACTIVATE",
            )
            .await;

            println!("[{}] - {:?}", line!(), outcome);

            async_io::Timer::after(Duration::from_millis(500)).await;
        }
    });
}
