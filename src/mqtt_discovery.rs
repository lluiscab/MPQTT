use crate::settings::MqttSettings;
use mqtt_async_client::client::{Client, Publish as PublishOpts, QoS};
use serde_derive::Serialize;

use log::info;

pub async fn run_mqtt_discovery(
    client: &Client,
    cfg: &MqttSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running MQTT Discovery");

    // Register error sensor
    register_error_sensor(client, cfg).await?;

    // Register serial number
    register_sensor(
        client,
        cfg,
        "qid",
        "serial_number",
        "Serial number",
        None,
        "slot-machine",
        None,
    )
    .await?;

    // TODO: Register protocol ID

    // Register QPIRI Sensors
    register_sensor(
        client,
        cfg,
        "qpiri",
        "grid_rating_voltage",
        "Grid Rating Voltage",
        Option::from(String::from("V")),
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "grid_rating_current",
        "Grid Rating Current",
        Option::from(String::from("A")),
        "current-dc",
        Option::from(false),
    )
    .await?;

    register_sensor(
        client,
        cfg,
        "qpiri",
        "ac_output_rating_voltage",
        "AC Output Rating Voltage",
        Option::from(String::from("V")),
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "ac_out_rating_frequency",
        "AC Output Rating Frequency",
        Option::from(String::from("Hz")),
        "current-ac",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "ac_out_rating_current",
        "AC Output Rating Current",
        Option::from(String::from("A")),
        "current-dc",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "ac_out_rating_apparent_power",
        "AC Output Rating Apparent Power",
        Option::from(String::from("W")),
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "ac_out_rating_active_power",
        "AC Output Rating Active Voltage",
        Option::from(String::from("W")),
        "power-plug",
        Option::from(false),
    )
    .await?;

    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_rating_voltage",
        "Battery Rating Voltage",
        Option::from(String::from("V")),
        "current-dc",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_recharge_voltage",
        "Battery Recharge Voltage",
        Option::from(String::from("V")),
        "current-dc",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_under_voltage",
        "Battery Under Voltage",
        Option::from(String::from("V")),
        "current-dc",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_bulk_voltage",
        "Battery Bulk Voltage",
        Option::from(String::from("V")),
        "current-dc",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_float_voltage",
        "Battery Float Voltage",
        Option::from(String::from("V")),
        "current-dc",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_redischarge_voltage",
        "Battery Redischarge Voltage",
        Option::from(String::from("V")),
        "battery-negative",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "battery_type",
        "Battery Type",
        None,
        "battery",
        Option::from(false),
    )
    .await?;

    register_sensor(
        client,
        cfg,
        "qpiri",
        "max_ac_charging_current",
        "Max AC Charging Current",
        Option::from(String::from("A")),
        "current-ac",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "max_charging_current",
        "Max Charging Current",
        Option::from(String::from("A")),
        "current-ac",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "input_voltage_range",
        "Input Voltage range",
        None,
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "output_source_priority",
        "Output Source Priority",
        None,
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "charge_source_priority",
        "Charge Source Priority",
        None,
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "machine_type",
        "Machine Type",
        None,
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "topology",
        "Topology",
        None,
        "power-plug",
        Option::from(false),
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpiri",
        "output_mode",
        "Output mode",
        None,
        "power-plug",
        Option::from(false),
    )
    .await?;

    // Register QPIGS Sensors
    register_sensor(
        client,
        cfg,
        "qpigs",
        "grid_voltage",
        "Grid Voltage",
        Option::from(String::from("V")),
        "power-plug",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "grid_frequency",
        "Grid Frequency",
        Option::from(String::from("Hz")),
        "current-ac",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "ac_out_voltage",
        "Out Voltage",
        Option::from(String::from("V")),
        "power-plug",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "ac_out_frequency",
        "Out Frequency",
        Option::from(String::from("Hz")),
        "current-ac",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "ac_out_apparent_power",
        "Out apparent power",
        Option::from(String::from("W")),
        "power-plug",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "ac_out_active_power",
        "Out active power",
        Option::from(String::from("W")),
        "power-plug",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "out_load_percent",
        "Out load percent",
        Option::from(String::from("%")),
        "brightness-percent",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "bus_voltage",
        "Bus Voltage",
        Option::from(String::from("V")),
        "details",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "battery_voltage",
        "Battery Voltage",
        Option::from(String::from("V")),
        "battery-outline",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "battery_charge_current",
        "Battery charge current",
        Option::from(String::from("A")),
        "current-dc",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "battery_capacity",
        "Battery capacity",
        Option::from(String::from("%")),
        "battery-outline",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "inverter_heat_sink_temp",
        "Heat sink temperature",
        Option::from(String::from("°C")),
        "details",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "pv_input_current",
        "PV Input Current",
        Option::from(String::from("A")),
        "solar-power",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "pv_input_voltage",
        "PV Input Voltage",
        Option::from(String::from("V")),
        "solar-power",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "battery_scc_voltage",
        "Battery SCC Voltage",
        Option::from(String::from("V")),
        "current-dc",
        None,
    )
    .await?;
    register_sensor(
        client,
        cfg,
        "qpigs",
        "battery_discharge_current",
        "Battery discharge current",
        Option::from(String::from("A")),
        "battery-negative",
        None,
    )
    .await?;

    /*
       device_status_active_load       PAYLOAD ON / PAYLOAD OFF
       device_status_charge_status
    */

    // TODO: Register QPIRI response
    // TODO: Register QPIWS response
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
        model: "MasterPower QPI".parse().unwrap(),
        manufacturer: "MasterPower QPI".parse().unwrap(),
        sw_version: "0.1.0".parse().unwrap(),
    }
}

async fn register_error_sensor(
    client: &Client,
    cfg: &MqttSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Registering error sensor");
    let params = SensorDiscoveryParams {
        unique_id: format!("{}_last_error", cfg.discovery.node_name)
            .parse()
            .unwrap(),
        name: "Last error".parse().unwrap(),
        unit_of_measurement: None,
        value_template: None,
        state_topic: format!("{}/{}", cfg.topic, "error").to_string(),
        icon: "mdi:hammer-wrench".parse().unwrap(),
        device: get_device_hassio(&cfg),
        force_update: true,
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(
        format!(
            "{}/sensor/{}/{}/config",
            cfg.discovery.prefix, cfg.discovery.node_name, "error"
        )
        .to_string(),
        params_string.as_bytes().to_vec(),
    );
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    client.publish(&msg).await?;
    Ok(())
}

async fn register_sensor(
    client: &Client,
    cfg: &MqttSettings,
    command: &str,
    id: &str,
    name: &str,
    unit: Option<String>,
    icon: &str,
    mut update: Option<bool>,
) -> Result<(), Box<dyn std::error::Error>> {
    let unique_id = format!("{}_{}", cfg.discovery.node_name, id)
        .parse()
        .unwrap();
    info!("Registering sensor {}", unique_id);
    let params = SensorDiscoveryParams {
        unique_id,
        name: String::from(name), // TODO: Should have masterpower prefix or something
        unit_of_measurement: unit,
        value_template: Option::from(format!("{{{{ value_json.{} }}}}", id).to_string()),
        state_topic: format!("{}/{}", cfg.topic, command).to_string(),
        icon: format!("mdi:{}", icon).to_string(),
        device: get_device_hassio(&cfg),
        force_update: *update.get_or_insert(true),
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(
        format!(
            "{}/sensor/{}/{}/config",
            cfg.discovery.prefix, cfg.discovery.node_name, id
        )
        .to_string(),
        params_string.as_bytes().to_vec(),
    );
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    client.publish(&msg).await?;
    Ok(())
}
