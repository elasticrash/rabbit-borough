# Rabbit-Borough

A rabbit MQ abstraction build upon [Lapin](https://crates.io/crates/lapin/1.0.2)

## Example

```rust
fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    LocalPool::new().run_until(async {
        consume(&config, &handler).await;
    })
}

fn handler(_delivery: &Delivery) -> HandleMessageResult {
    // In order to read the message you need to convert the _delivery.data from a u8 vec to a utf8 string :
    // std::str::from_utf8(&_delivery.data))
    return HandleMessageResult::Ack;
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
        }
    },
    "declare": {
        "queue": true,
        "exchange": true,
        "binding": true
    }
}
```




## Idea

The whole idea is basically to be able to create a consumer project with minimal effort, by bypassing templating, configuration and complicated resiliency logic. 

But most of the modules are public in this abstraction, so as to left some breathing space for custom composing. 

## Thoughts
Given that I use rabbitMq daily in nearly every application, this mini library is something that I might benefit in the near future. Luckily someone could find similar benefit as well.