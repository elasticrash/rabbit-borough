use crate::configuration::config_model::JSONConfiguration;
use std::fs;
use std::path::Path;

/**
 * ## Reads configuration from provided file
 * If file does not exist or we cannot read the content of the file,
 * we use the default values.
 * If the file exists but is not valid JSON, the parse error is returned.
 */
pub fn read(filename: &str) -> serde_json::Result<JSONConfiguration> {
    match fs::read_to_string(filename) {
        Ok(buffer) => {
            println!(
                "[{}] - Reading {:?}",
                line!(),
                Path::new(filename).file_name()
            );
            serde_json::from_str(&buffer)
        }
        Err(_why) => Ok(JSONConfiguration::default()),
    }
}

#[cfg(test)]
mod tests {

    use crate::configuration::reader::*;

    #[test]
    fn get_default_config_if_file_does_not_exist() {
        assert_eq!(read("random").unwrap(), JSONConfiguration::default());
    }

    #[test]
    fn partial_config_falls_back_to_defaults() {
        let config: JSONConfiguration =
            serde_json::from_str(r#"{"connection": {"host": "rabbit"}}"#).unwrap();
        assert_eq!(config.connection.host, "rabbit");
        assert_eq!(config.connection.port, 5672);
        assert_eq!(config.binding.queue, "myQueue");
    }
}
