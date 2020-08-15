use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InverterSettings {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct MqttSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub inverter: InverterSettings,
    pub mqtt: MqttSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.merge(File::with_name("config"))?;
        settings.try_into()
    }
}
