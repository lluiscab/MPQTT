use crate::settings::MqttSettings;
use mqtt_async_client::client::{Client, Publish as PublishOpts, QoS};
use serde_derive::Serialize;

use log::info;

pub async fn run_mqtt_discovery(client: &Client, cfg: &MqttSettings) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running MQTT Discovery");

    // Register error sensor
    register_error_sensor(client, cfg).await?;

    // Register QID Response
    register_sensor(client, cfg, "qid", "serial_number", "Serial number", None, "slot-machine", Some(false)).await?;

    // Register QPI Response
    register_sensor(client, cfg, "qpi", "protocol_id", "Protocol ID", None, "slot-machine", Some(false)).await?;

    // TODO: Register software version1
    // TODO: Register software version2

    // TODO: Register QMOD

    // Register QPIRI Sensors
    register_sensor(client, cfg, "qpiri", "grid_rating_voltage", "Grid Rating Voltage", Some("V".to_string()), "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "grid_rating_current", "Grid Rating Current", Some("A".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "ac_output_rating_voltage", "AC Output Rating Voltage", Some("V".to_string()), "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_frequency", "AC Output Rating Frequency", Some("Hz".to_string()), "current-ac", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_current", "AC Output Rating Current", Some("A".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_apparent_power", "AC Output Rating Apparent Power", Some("W".to_string()), "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_active_power", "AC Output Rating Active Voltage", Some("W".to_string()), "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_rating_voltage", "Battery Rating Voltage", Some("V".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_recharge_voltage", "Battery Recharge Voltage", Some("V".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_under_voltage", "Battery Under Voltage", Some("V".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_bulk_voltage", "Battery Bulk Voltage", Some("V".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_float_voltage", "Battery Float Voltage", Some("V".to_string()), "current-dc", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_redischarge_voltage", "Battery Redischarge Voltage", Some("V".to_string()), "battery-negative", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "battery_type", "Battery Type", None, "battery", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "max_ac_charging_current", "Max AC Charging Current", Some("A".to_string()), "current-ac", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "max_charging_current", "Max Charging Current", Some("A".to_string()), "current-ac", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "input_voltage_range", "Input Voltage range", None, "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "output_source_priority", "Output Source Priority", None, "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "charge_source_priority", "Charge Source Priority", None, "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "machine_type", "Machine Type", None, "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "topology", "Topology", None, "power-plug", Some(false)).await?;
    register_sensor(client, cfg, "qpiri", "output_mode", "Output mode", None, "power-plug", Some(false)).await?;

    // Register QPIGS Sensors
    register_sensor(client, cfg, "qpigs", "grid_voltage", "Grid Voltage", Some("V".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpigs", "grid_frequency", "Grid Frequency", Some("Hz".to_string()), "current-ac", None).await?;
    register_sensor(client, cfg, "qpigs", "ac_out_voltage", "Out Voltage", Some("V".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpigs", "ac_out_frequency", "Out Frequency", Some("Hz".to_string()), "current-ac", None).await?;
    register_sensor(client, cfg, "qpigs", "ac_out_apparent_power", "Out apparent power", Some("W".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpigs", "ac_out_active_power", "Out active power", Some("W".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpigs", "out_load_percent", "Out load percent", Some("%".to_string()), "brightness-percent", None).await?;
    register_sensor(client, cfg, "qpigs", "bus_voltage", "Bus Voltage", Some("V".to_string()), "details", None).await?;
    register_sensor(client, cfg, "qpigs", "battery_voltage", "Battery Voltage", Some("V".to_string()), "battery-outline", None).await?;
    register_sensor(client, cfg, "qpigs", "battery_charge_current", "Battery charge current", Some("A".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpigs", "battery_capacity", "Battery capacity", Some("%".to_string()), "battery-outline", None).await?;
    register_sensor(client, cfg, "qpigs", "inverter_heat_sink_temp", "Heat sink temperature", Some("°C".to_string()), "details", None).await?;
    register_sensor(client, cfg, "qpigs", "pv_input_current", "PV Input Current", Some("A".to_string()), "solar-power", None).await?;
    register_sensor(client, cfg, "qpigs", "pv_input_voltage", "PV Input Voltage", Some("V".to_string()), "solar-power", None).await?;
    register_sensor(client, cfg, "qpigs", "battery_scc_voltage", "Battery SCC Voltage", Some("V".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpigs", "battery_discharge_current", "Battery discharge current", Some("A".to_string()), "battery-negative", None).await?;
    register_sensor(client, cfg, "qpigs", "device_status.charge_status", "Device charge status", None, "power-plug", None).await?;
    register_sensor(client, cfg, "qpigs", "device_status.active_load", "Active load", None, "power", None).await?;

    // TODO: Register QPIWS response
    // TODO: Register QFLAG response
    Ok(())
}

#[derive(Serialize, Debug)]
struct SensorDiscoveryParams {
    unique_id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_of_measurement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_template: Option<String>,
    state_topic: String,
    icon: String,
    device: SensorDiscoveryDevice,
    force_update: bool,
}

#[derive(Serialize, Debug)]
struct SensorDiscoveryDevice {
    name: String,
    identifiers: [String; 1],
    model: String,
    manufacturer: String,
    sw_version: String,
}

fn get_device_hassio(cfg: &MqttSettings) -> SensorDiscoveryDevice {
    SensorDiscoveryDevice {
        name: cfg.discovery.device_name.clone(),
        identifiers: [cfg.discovery.device_id.clone()],
        model: env!("CARGO_PKG_NAME").to_ascii_uppercase().parse().unwrap(),
        manufacturer: env!("CARGO_PKG_NAME").to_ascii_uppercase().parse().unwrap(),
        sw_version: env!("CARGO_PKG_VERSION").parse().unwrap(),
    }
}

async fn register_error_sensor(client: &Client, cfg: &MqttSettings) -> Result<(), Box<dyn std::error::Error>> {
    info!("Registering error sensor");
    let params = SensorDiscoveryParams {
        unique_id: format!("{}_last_error", cfg.discovery.node_name).parse().unwrap(),
        name: format!("{} - Last error", cfg.discovery.device_name).to_string(),
        unit_of_measurement: None,
        value_template: None,
        state_topic: format!("{}/{}", cfg.topic, "error").to_string(),
        icon: "mdi:hammer-wrench".parse().unwrap(),
        device: get_device_hassio(&cfg),
        force_update: true,
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(format!("{}/sensor/{}/{}/config", cfg.discovery.prefix, cfg.discovery.node_name, "error").to_string(), params_string.as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(true);
    client.publish(&msg).await?;
    Ok(())
}

async fn register_sensor(client: &Client, cfg: &MqttSettings, command: &str, id: &str, name: &str, unit: Option<String>, icon: &str, mut update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    let unique_id = format!("{}_{}", cfg.discovery.node_name, id).to_string().replace(".", "_");

    info!("Registering sensor {}", unique_id);
    let params = SensorDiscoveryParams {
        unique_id,
        name: format!("{} - {}", cfg.discovery.device_name, name).to_string(),
        unit_of_measurement: unit,
        value_template: Some(format!("{{{{ value_json.{} }}}}", id).to_string()),
        state_topic: format!("{}/{}", cfg.topic, command).to_string(),
        icon: format!("mdi:{}", icon).to_string(),
        device: get_device_hassio(&cfg),
        force_update: *update.get_or_insert(true),
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(format!("{}/sensor/{}/{}/config", cfg.discovery.prefix, cfg.discovery.node_name, id.replace(".", "_")).to_string(), params_string.as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(true);
    client.publish(&msg).await?;
    Ok(())
}
