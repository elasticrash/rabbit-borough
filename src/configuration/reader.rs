extern crate serde;
use crate::configuration::config_model::JSONConfiguration;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/**
 * ## Read configuration from config.json
 * If file does not exist or we cannot read the content of the file,
 * we use the default values
 */
pub fn read(filename: &str) -> serde_json::Result<JSONConfiguration> {
    let mut buffer = String::new();
    let mut config = Ok(JSONConfiguration::default());
    match File::open(filename) {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
            config = serde_json::from_str(&buffer);
            println!("[{}] - Reading {:?}", line!(), Path::new(filename).file_name());
        }
        Err(_why) => {}
    };

    return config;
}
