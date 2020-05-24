use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct JSONConfiguration {
    pub address: String,
}