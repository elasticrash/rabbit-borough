# Rabbit-Borough
## A rabbit MQ abstraction build upon lapin
![Rust](https://github.com/elasticrash/rabbit-borough/workflows/Rust/badge.svg)

Goal: To fit my needs and hopefully, someone else's

(work in progress) my plan is to add a bit of code at least 2-4 times per week (until I reach a satisfying point)

## Consumer example

```rust
fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        let why = consume(&config, &handler).await;
        println!("giving up: {:?}", why);
    })
}

fn handler(_delivery: &DeliveredMessage) -> HandleMessageResult {
    HandleMessageResult::Ack
}
```

## Publisher example

```rust
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

            async_io::Timer::after(Duration::from_millis(500)).await;
        }
    });
}
```

## JSONConfiguration configuration example


The entire configuration supports default implementations. So if the default assumptions are right for you don't need to provide the entire config, only the parts you are interested in.

This is a full example 
```json
{
    "connection": {
        "host": "127.0.0.1",
        "port": 5672,
        "vhost": "/",
        "heartbeat": 10,
        "connection_timeout": 1000,
        "username": "secure",
        "password": "secure",
        "retry": 4
    },
    "binding": {
        "queue": "myQueue",
        "exchange": "myExchange",
        "routing_key": "myKey",
        "exchange_declaration_options": {
            "passive": false,
            "durable": true,
            "auto_delete": false
        },
        "queue_declaration_options": {
            "passive": false,
            "durable": true,
            "exclusive": false,
            "auto_delete": false
        }
    },
    "declare": {
        "queue": true,
        "exchange": true,
        "binding": true
    }
}
```



## IDEA

The whole idea is basically to be able to create a consumer or publisher project with minimal effort, by bypassing templating, configuration and complicated resiliency logic. 

The only thing, that will eventually needed is to write the message handler logic 
