extern crate rabbit_borough;

use futures_executor::LocalPool;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use rabbit_borough::configuration;
use rabbit_borough::configuration::config_model::JSONConfiguration;
use rabbit_borough::consumer::consumer::consume_with_option;
use rabbit_borough::consumer::consumer::DeliveredMessage;
use rabbit_borough::consumer::handle_message_result::HandleMessageResult;

#[derive(Debug)]
struct ConsumerProperties {
    db_pool: Pool<PostgresConnectionManager<NoTls>>,
}

fn main() {
    let config: JSONConfiguration = configuration::reader::read("./config.json").unwrap();
    println!("[{}] - Configuration read", line!(),);

    let manager = PostgresConnectionManager::new(
        "postgresql://postgres:postgres@localhost:5432/test_db"
            .parse()
            .unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).unwrap();

    LocalPool::new().run_until(async {
        consume_with_option(
            &config,
            &handler,
            Some(&ConsumerProperties { db_pool: pool }),
        )
        .await;
    })
}

// function to handle the message
// follows db script of this example
//
//
//  create table if not exists random_table
// (
//	c1 integer,
//	c2 integer,
//	id serial not null
// );
//
// create unique index if not exists random_table_id_uindex
//	on random_table (id);
fn handler(
    _delivery: &DeliveredMessage,
    _prop: Option<&ConsumerProperties>,
) -> HandleMessageResult {
    // CONSUMER LOGIC
    println!("[{}] - {:?}", line!(), std::str::from_utf8(&_delivery.data));
    println!("[{}] - {:?}", line!(), _prop);

    let mut client = _prop.unwrap().db_pool.get().unwrap();
    client
        .execute(
            "INSERT INTO random_table (c1, c2) VALUES ($1, $2)",
            &[&1, &2],
        )
        .unwrap();

    return HandleMessageResult::Ack;
}
