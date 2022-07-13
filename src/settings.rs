use config::{Config, ConfigError, File};
use serde_derive::Deserialize;

#[cfg(not(feature = "build-for-deb"))]
const CONFIG_PATH: &'static str = "config.yaml";

#[cfg(feature = "build-for-deb")]
const CONFIG_PATH: &'static str = "/etc/mpqtt/config.yaml";

#[derive(Debug, Deserialize)]
pub struct InverterSettings {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct MqttDiscovery {
    pub prefix: String,
    pub node_name: String,
    pub device_name: String,
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MqttSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub topic: String,
    pub discovery: MqttDiscovery,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub low_priority_delay: i16,
    pub inverter: InverterSettings,
    pub mqtt: MqttSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        settings.merge(File::with_name(CONFIG_PATH))?;

        settings.try_into()
    }
}
