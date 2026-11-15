use crate::configuration::config_model::JSONConfiguration;
use crate::consumer::connection_manager::GenericError;
use crate::consumer::handle_message_result::HandleMessageResult;
use crate::consumer::handle_message_result::action_result;
use crate::consumer::setup::setup_consumer;
use async_io::Timer;
use futures_lite::StreamExt;
use lapin::Channel;
use lapin::Consumer;
use lapin::Error;
use lapin::message::Delivery;
use lapin::options::BasicConsumeOptions;
use lapin::types::FieldTable;
use std::time::Duration;

pub type DeliveredMessage = Delivery;

const RECONNECT_DELAY: Duration = Duration::from_secs(1);

/// # Create a consumer
/// Returns consumer
pub async fn create_consumer(queue_name: &str, channel: &Channel) -> Result<Consumer, Error> {
    channel
        .basic_consume(
            queue_name.into(),
            "".into(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
}

/// This is the function that the consuming is happening
/// There are two (infinite) loops here, a. one around the consumer, which
/// while the connection is healthy keeps dequeuing messages
/// b. one that once the connection dies, restarts the whole process of getting a channel and
/// setting up the consumer.
///
/// this function also allows you to pass an argument of type T into the handler
///
/// It only returns when a connection cannot be (re-)established after
/// exhausting all configured retries; the returned value describes why.
pub async fn consume_with_option<T>(
    config: &JSONConfiguration,
    handler: &dyn Fn(&Delivery, Option<&T>) -> HandleMessageResult,
    args: Option<&T>,
) -> GenericError<Error> {
    loop {
        let model = match setup_consumer(
            config.connection.clone(),
            config.binding.clone(),
            config.declare.clone(),
        )
        .await
        {
            Ok(model) => model,
            Err(why) => return why,
        };

        log_setup_error("queue_declare", &model.queue);
        log_setup_error("exchange_declare", &model.exchange);
        log_setup_error("queue_bind", &model.binding);

        println!(
            "[{}] - channel connected: {:?}",
            line!(),
            model.channel.status().connected()
        );

        match create_consumer(&config.binding.queue, &model.channel).await {
            Ok(mut consumer) => {
                println!("[{}] - {:?}", line!(), consumer.tag());

                while let Some(res_delivery) = consumer.next().await {
                    let delivery = match res_delivery {
                        Ok(delivery) => delivery,
                        Err(why) => {
                            println!("[{}] - delivery error: {:?}", line!(), why);
                            break;
                        }
                    };
                    if let Err(why) = action_result(
                        handler(&delivery, args),
                        &model.channel,
                        delivery.delivery_tag,
                    )
                    .await
                    {
                        println!("[{}] - acknowledgement error: {:?}", line!(), why);
                        break;
                    }
                }
            }
            Err(why) => println!("[{}] - basic_consume failed: {:?}", line!(), why),
        }

        let _ = model.connection.close(0, "reconnecting".into()).await;
        Timer::after(RECONNECT_DELAY).await;
    }
}

fn log_setup_error<T>(step: &str, outcome: &Option<Result<T, Error>>) {
    if let Some(Err(why)) = outcome {
        println!("[{}] - {} failed: {:?}", line!(), step, why);
    }
}

/// This is the function that the consuming is happening
/// There are two (infinite) loops here, a. one around the consumer, which
/// while the connection is healthy keeps dequeuing messages
/// b. one that once the connection dies, restarts the whole process of getting a channel and
/// setting up the consumer
///
/// It only returns when a connection cannot be (re-)established after
/// exhausting all configured retries; the returned value describes why.
pub async fn consume(
    config: &JSONConfiguration,
    handler: &dyn Fn(&Delivery) -> HandleMessageResult,
) -> GenericError<Error> {
    consume_with_option::<()>(config, &|delivery, _| handler(delivery), None).await
}
