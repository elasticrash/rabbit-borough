extern crate serde;
use crate::configuration::config_model::JSONConfiguration;
use std::fs::File;
use std::io::Read;

/**
 * Read configuration from config.json
 */
pub fn read(filename: &str) -> serde_json::Result<JSONConfiguration> {
    let mut file = match File::open(filename) {
        Ok(data) => data,
        Err(why) => {
            println!("error {}", why);
            panic!()
        }
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_d) => println!("...configuration read"),
        Err(why) => println!("error {}", why),
    }

    let config = serde_json::from_str(&buffer);
    return config;
}
