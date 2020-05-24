mod consumer;
use futures_executor::LocalPool;
use lapin::options::BasicAckOptions;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    let addr: &'static str = "amqp://127.0.0.1:5672/%2f?heartbeat=10&connection_timeout=1000";
    LocalPool::new().run_until(async {
        let model = consumer::setup::setup_consumer(addr).await;

        println!(
            "[{}] channel status: {:?}",
            line!(),
            model.channel.status().state()
        );

        println!("[{}] queue status: {:?}", line!(), model.queue);
        println!("[{}] exchange status: {:?}", line!(), model.exchange);
        println!("[{}] bind status: {:?}", line!(), model.binding);

        println!("will consume");
        let consumer = model
            .channel
            .basic_consume(
                "hello",
                "my_consumer",
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
