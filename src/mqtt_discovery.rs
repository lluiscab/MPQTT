use crate::settings::MqttSettings;
use mqtt_async_client::client::{Client, Publish as PublishOpts, QoS};
use serde_derive::Serialize;

use log::info;

pub async fn run_mqtt_discovery(client: &Client, cfg: &MqttSettings) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running MQTT Discovery");

    // Register error sensor
    register_error_sensor(client, cfg).await?;

    // Register update sensor
    register_sensor(client, cfg, "update", "timestamp", "Last update", None, "calendar", None).await?;
    register_sensor(client, cfg, "update", "execution_time", "Last update execution time", Some("ms".to_string()), "clock-outline", None).await?;

    // Register QID Response
    register_sensor(client, cfg, "qid", "serial_number", "Serial number", None, "slot-machine", None).await?;

    // Register QPI Response
    register_sensor(client, cfg, "qpi", "protocol_id", "Protocol ID", None, "slot-machine", None).await?;

    // Register software version1
    register_sensor(client, cfg, "qvfw", "major", "CPU Firmware Version Major", None, "update", None).await?;
    register_sensor(client, cfg, "qvfw", "minor", "CPU Firmware Version Minor", None, "update", None).await?;

    // Register software version 22
    register_sensor(client, cfg, "qvfw2", "major", "CPU Firmware Version 2 Major", None, "update", None).await?;
    register_sensor(client, cfg, "qvfw2", "minor", "CPU Firmware Version 2 Minor", None, "update", None).await?;

    // Register QMOD
    register_sensor(client, cfg, "qmod", "mode", "Device mode", None, "information-outline", None).await?;

    // Register QPIRI Sensors
    register_sensor(client, cfg, "qpiri", "grid_rating_voltage", "Grid Rating Voltage", Some("V".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "grid_rating_current", "Grid Rating Current", Some("A".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "ac_output_rating_voltage", "AC Output Rating Voltage", Some("V".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_frequency", "AC Output Rating Frequency", Some("Hz".to_string()), "current-ac", None).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_current", "AC Output Rating Current", Some("A".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_apparent_power", "AC Output Rating Apparent Power", Some("W".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_active_power", "AC Output Rating Active Voltage", Some("W".to_string()), "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_rating_voltage", "Battery Rating Voltage", Some("V".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_recharge_voltage", "Battery Recharge Voltage", Some("V".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_under_voltage", "Battery Under Voltage", Some("V".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_bulk_voltage", "Battery Bulk Voltage", Some("V".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_float_voltage", "Battery Float Voltage", Some("V".to_string()), "current-dc", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_redischarge_voltage", "Battery Redischarge Voltage", Some("V".to_string()), "battery-negative", None).await?;
    register_sensor(client, cfg, "qpiri", "battery_type", "Battery Type", None, "battery", None).await?;
    register_sensor(client, cfg, "qpiri", "max_ac_charging_current", "Max AC Charging Current", Some("A".to_string()), "current-ac", None).await?;
    register_sensor(client, cfg, "qpiri", "max_charging_current", "Max Charging Current", Some("A".to_string()), "current-ac", None).await?;
    register_sensor(client, cfg, "qpiri", "input_voltage_range", "Input Voltage range", None, "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "output_source_priority", "Output Source Priority", None, "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "charge_source_priority", "Charge Source Priority", None, "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "machine_type", "Machine Type", None, "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "topology", "Topology", None, "power-plug", None).await?;
    register_sensor(client, cfg, "qpiri", "output_mode", "Output mode", None, "power-plug", None).await?;

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
    register_sensor(client, cfg, "qpigs", "device_status.active_load", "Active load", None, "power", None).await?;

    register_sensor(client, cfg, "qpiws", "inverter_fault", "Inverter fault", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "bus_over", "Bus over", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "bus_under", "Bus under", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "bus_soft_fail", "Bus soft fail", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "line_fail", "Line fail", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "opv_short", "OPV Short", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "inverter_voltage_too_low", "Inverter voltage too low", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "inverter_voltage_too_high", "Inverter voltage too high", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "over_temperature", "Over temperature", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "fan_locked", "Fan locked", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "battery_voltage_high", "Battery voltage high", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "battery_low_alarm", "Battery low alarm", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "battery_under_shutdown", "Battery under shutdown", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "over_load", "Over load", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "eeprom_fault", "EEPROM Fault", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "inverter_over_current", "Inverter over current", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "inverter_soft_fail", "Inverter soft fail", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "self_test_fail", "Self test fail", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "op_dc_voltage_over", "OP DC Voltage over", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "bat_open", "Bat open", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "current_sensor_fail", "Current sensor fail", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "battery_short", "Battery short", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "power_limit", "Power limit", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "pv_voltage_high", "PV Voltage high", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "mppt_overload_fault", "MPPT Overload fault", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "mppt_overload_warning", "MPPT Overload warning", None, "alert", None).await?;
    register_sensor(client, cfg, "qpiws", "battery_too_low_to_charge", "Battery too low to charge", None, "alert", None).await?;

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
        force_update: false,
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(format!("{}/sensor/{}/{}/config", cfg.discovery.prefix, cfg.discovery.node_name, "error").to_string(), params_string.as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(true);
    client.publish(&msg).await?;
    Ok(())
}

async fn register_sensor(client: &Client, cfg: &MqttSettings, command: &str, id: &str, name: &str, unit: Option<String>, icon: &str, mut force_update: Option<bool>) -> Result<(), Box<dyn std::error::Error>> {
    let unique_id = format!("{}_{}_{}", cfg.discovery.node_name, command, id).to_string().replace(".", "_");
    let topic = format!("{}/{}", cfg.topic, command).to_string();

    info!("Registering sensor {}", unique_id);
    let params = SensorDiscoveryParams {
        unique_id,
        name: format!("{} - {}", cfg.discovery.device_name, name).to_string(),
        unit_of_measurement: unit,
        value_template: Some(format!("{{{{ value_json.{} }}}}", id).to_string()),
        state_topic: topic,
        icon: format!("mdi:{}", icon).to_string(),
        device: get_device_hassio(&cfg),
        force_update: *force_update.get_or_insert(false),
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(format!("{}/sensor/{}/{}_{}/config", cfg.discovery.prefix, cfg.discovery.node_name, command, id.replace(".", "_")).to_string(), params_string.as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(true);
    client.publish(&msg).await?;
    Ok(())
}
