use crate::settings::MqttSettings;
use mqtt_async_client::client::{Client, Publish as PublishOpts, QoS};
use serde_derive::Serialize;

use log::{debug, info};

pub async fn run_mqtt_discovery(client: &Client, cfg: &MqttSettings, inverter_count: u8, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Running MQTT Discovery");

    // Register error sensor
    register_error_sensor(client, cfg).await?;

    // Register stats sensors
    register_sensor(client, cfg, "inner_stats", "update_duration", "Last Inner Update Duration", Some("ms".to_string()), "clock").await?;
    register_sensor(client, cfg, "outer_stats", "update_duration", "Last Outer Update Duration", Some("ms".to_string()), "clock").await?;

    // Register QID Response
    register_sensor(client, cfg, "qid", "serial_number", "Serial number", None, "slot-machine").await?;

    // Register QPI Response
    register_sensor(client, cfg, "qpi", "protocol_id", "Protocol ID", None, "slot-machine").await?;

    // Register software version 1
    register_sensor(client, cfg, "qvfw", "major", "CPU Firmware Version Major", None, "update").await?;
    register_sensor(client, cfg, "qvfw", "minor", "CPU Firmware Version Minor", None, "update").await?;

    // Register software version 2
    register_sensor(client, cfg, "qvfw2", "major", "CPU Firmware Version 2 Major", None, "update").await?;
    register_sensor(client, cfg, "qvfw2", "minor", "CPU Firmware Version 2 Minor", None, "update").await?;

    // Register software version 3
    if mode != "phocos" {
        register_sensor(client, cfg, "qvfw3", "major", "CPU Firmware Version 2 Major", None, "update").await?;
        register_sensor(client, cfg, "qvfw3", "minor", "CPU Firmware Version 2 Minor", None, "update").await?;
    }

    // Register QMOD
    register_sensor(client, cfg, "qmod", "mode", "Device mode", None, "information-outline").await?;

    // Register QPIRI Sensors
    register_sensor(client, cfg, "qpiri", "grid_rating_voltage", "Grid Rating Voltage", Some("V".to_string()), "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "grid_rating_current", "Grid Rating Current", Some("A".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "ac_output_rating_voltage", "AC Output Rating Voltage", Some("V".to_string()), "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_frequency", "AC Output Rating Frequency", Some("Hz".to_string()), "current-ac").await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_current", "AC Output Rating Current", Some("A".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_apparent_power", "AC Output Rating Apparent Power", Some("W".to_string()), "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "ac_out_rating_active_power", "AC Output Rating Active Voltage", Some("W".to_string()), "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "battery_rating_voltage", "Battery Rating Voltage", Some("V".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "battery_recharge_voltage", "Battery Recharge Voltage", Some("V".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "battery_under_voltage", "Battery Under Voltage", Some("V".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "battery_bulk_voltage", "Battery Bulk Voltage", Some("V".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "battery_float_voltage", "Battery Float Voltage", Some("V".to_string()), "current-dc").await?;
    register_sensor(client, cfg, "qpiri", "battery_redischarge_voltage", "Battery Redischarge Voltage", Some("V".to_string()), "battery-negative").await?;
    register_sensor(client, cfg, "qpiri", "battery_type", "Battery Type", None, "battery").await?;
    register_sensor(client, cfg, "qpiri", "max_ac_charging_current", "Max AC Charging Current", Some("A".to_string()), "current-ac").await?;
    register_sensor(client, cfg, "qpiri", "max_charging_current", "Max Charging Current", Some("A".to_string()), "current-ac").await?;
    register_sensor(client, cfg, "qpiri", "input_voltage_range", "Input Voltage range", None, "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "output_source_priority", "Output Source Priority", None, "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "charge_source_priority", "Charge Source Priority", None, "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "machine_type", "Machine Type", None, "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "topology", "Topology", None, "power-plug").await?;
    register_sensor(client, cfg, "qpiri", "output_mode", "Output mode", None, "power-plug").await?;

    // Register QPIGS Sensors
    if mode != "phocos" {
        register_sensor(client, cfg, "qpigs", "grid_voltage", "Grid Voltage", Some("V".to_string()), "power-plug").await?;
        register_sensor(client, cfg, "qpigs", "grid_frequency", "Grid Frequency", Some("Hz".to_string()), "current-ac").await?;
        register_sensor(client, cfg, "qpigs", "ac_out_voltage", "Out Voltage", Some("V".to_string()), "power-plug").await?;
        register_sensor(client, cfg, "qpigs", "ac_out_frequency", "Out Frequency", Some("Hz".to_string()), "current-ac").await?;
        register_sensor(client, cfg, "qpigs", "ac_out_apparent_power", "Out apparent power", Some("W".to_string()), "power-plug").await?;
        register_sensor(client, cfg, "qpigs", "ac_out_active_power", "Out active power", Some("W".to_string()), "power-plug").await?;
        register_sensor(client, cfg, "qpigs", "out_load_percent", "Out load percent", Some("%".to_string()), "brightness-percent").await?;
        register_sensor(client, cfg, "qpigs", "bus_voltage", "Bus Voltage", Some("V".to_string()), "details").await?;
        register_sensor(client, cfg, "qpigs", "battery_voltage", "Battery Voltage", Some("V".to_string()), "battery-outline").await?;
        register_sensor(client, cfg, "qpigs", "battery_charge_current", "Battery charge current", Some("A".to_string()), "current-dc").await?;
        register_sensor(client, cfg, "qpigs", "battery_capacity", "Battery capacity", Some("%".to_string()), "battery-outline").await?;
        register_sensor(client, cfg, "qpigs", "inverter_heat_sink_temp", "Heat sink temperature", Some("°C".to_string()), "details").await?;
        register_sensor(client, cfg, "qpigs", "pv_input_current", "PV Input Current", Some("A".to_string()), "solar-power").await?;
        register_sensor(client, cfg, "qpigs", "pv_input_voltage", "PV Input Voltage", Some("V".to_string()), "solar-power").await?;
        register_sensor(client, cfg, "qpigs", "battery_scc_voltage", "Battery SCC Voltage", Some("V".to_string()), "current-dc").await?;
        register_sensor(client, cfg, "qpigs", "battery_discharge_current", "Battery discharge current", Some("A".to_string()), "battery-negative").await?;
        register_sensor(client, cfg, "qpigs", "device_status.charge_status", "Device charge status", None, "power-plug").await?;
        register_sensor(client, cfg, "qpigs", "device_status.active_load", "Active load", None, "power").await?;
    }

    assert_ne!(inverter_count, 0);
    for index in 0..=inverter_count {
        // Register QPGS Sensors
        register_sensor(client, cfg, &format!("qpgs{}", index), "other_units_connected", &format!("Other Units Connected - Inverter {}", index), None, "power-plug").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "serial_number", &format!("Serial Number - Inverter {}", index), None, "details").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "operation_mode", &format!("Operation Mode - Inverter {}", index), None, "slot-machine").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "fault_code", &format!("Fault Code - Inverter {}", index), None, "alert").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_input_voltage", &format!("AC Input Voltage - Inverter {}", index), Some("Vac".to_string()), "power-plug").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_input_frequency", &format!("AC Input Frequency - Inverter {}", index), Some("Hz".to_string()), "current-ac").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_output_voltage", &format!("AC Output Voltage - Inverter {}", index), Some("Vac".to_string()), "power-plug").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_output_frequency", &format!("AC Output Frequency - Inverter {}", index), Some("Hz".to_string()), "current-ac").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_output_apparent_power", &format!("AC Output Apparent Power - Inverter {}", index), Some("VA".to_string()), "power-plug").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_output_active_power", &format!("AC Output Active Power - Inverter {}", index), Some("W".to_string()), "power-plug").await?;
        register_sensor(
            client,
            cfg,
            &format!("qpgs{}", index),
            "percentage_of_nominal_output_power",
            &format!("Percentage Of Nominal Output Power - Inverter {}", index),
            Some("% of single inverter".to_string()),
            "power-plug",
        )
        .await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_voltage", &format!("Battery Votlage - Inverter {}", index), Some("Vdc".to_string()), "battery").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_charging_current", &format!("Battery Charging Current - Inverter {}", index), Some("Adc".to_string()), "battery-positive").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_approx_state_of_charge", &format!("Battery State of Charge - Inverter {}", index), Some("%".to_string()), "battery-outline").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "pv_input_voltage", &format!("PV Input Voltage - Inverter {}", index), Some("Vdc".to_string()), "solar-power").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "total_charging_current", &format!("Total Charging Current - Inverter {}", index), Some("Adc".to_string()), "battery-positive").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "total_ac_output_apparent_power", &format!("Total AC Output Apparent Power - Inverter {}", index), Some("VA".to_string()), "power-plug").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "total_ac_output_active_power", &format!("Total AC Output Active Power - Inverter {}", index), Some("W".to_string()), "power-plug").await?;
        register_sensor(
            client,
            cfg,
            &format!("qpgs{}", index),
            "total_percentage_of_nominal_output_power",
            &format!("Total Percentage Of Output Power - Inverter {}", index),
            Some("% of inverters".to_string()),
            "power-plug",
        )
        .await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.mppt_active", &format!("MPPT Active - Inverter {}", index), None, "order-bool-ascending-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.ac_charging", &format!("AC Charging - Inverter {}", index), None, "order-bool-ascending-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.solar_charging", &format!("Solar Charging - Inverter {}", index), None, "order-bool-ascending-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.battery_status", &format!("Battery Status - Inverter {}", index), None, "battery-heart-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.ac_input", &format!("AC Input - Inverter {}", index), None, "order-bool-ascending-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.ac_output", &format!("AC Output - Inverter {}", index), None, "order-bool-ascending-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "inverter_status.reserved_bit", &format!("Reserved - Inverter {}", index), None, "order-bool-ascending-variant").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "ac_output_mode", &format!("AC Output Mode - Inverter {}", index), None, "slot-machine").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_charging_source_priority", &format!("Battery Charging Source - Inverter {}", index), None, "ev-station").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "max_charging_current_set", &format!("Max Charging Current Set - Inverter {}", index), Some("Adc".to_string()), "current-dc").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "max_charging_current_possible", &format!("Max Charging Current Possible - Inverter {}", index), Some("Adc".to_string()), "current-dc").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "max_ac_charging_current_set", &format!("Max AC Charging Current Set - Inverter {}", index), Some("Adc".to_string()), "current-dc").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "pv_input_current", &format!("PV Input Current - Inverter {}", index), Some("Adc".to_string()), "current-dc").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_discharge_current", &format!("Battery Discharge Current - Inverter {}", index), Some("Adc".to_string()), "current-dc").await?;

        // manually calculated - not reported from qpgs directly
        register_sensor(client, cfg, &format!("qpgs{}", index), "pv_input_power", &format!("PV Input Power - Inverter {}", index), Some("W".to_string()), "solar-panel").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_charging_power", &format!("Battery Charging Power - Inverter {}", index), Some("W".to_string()), "battery-positive").await?;
        register_sensor(client, cfg, &format!("qpgs{}", index), "battery_discharging_power", &format!("Battery Discharging Power - Inverter {}", index), Some("W".to_string()), "battery-negative").await?;
    }

    // Register QPIWS response

    register_sensor(client, cfg, "qpiws", "inverter_fault", "Inverter fault", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "bus_over", "Bus over", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "bus_under", "Bus under", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "bus_soft_fail", "Bus soft fail", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "line_fail", "Line fail", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "opv_short", "OPV Short", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "inverter_voltage_too_low", "Inverter voltage too low", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "inverter_voltage_too_high", "Inverter voltage too high", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "over_temperature", "Over temperature", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "fan_locked", "Fan locked", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "battery_voltage_high", "Battery voltage high", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "battery_low_alarm", "Battery low alarm", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "battery_under_shutdown", "Battery under shutdown", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "over_load", "Over load", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "eeprom_fault", "EEPROM Fault", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "inverter_over_current", "Inverter over current", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "inverter_soft_fail", "Inverter soft fail", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "self_test_fail", "Self test fail", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "op_dc_voltage_over", "OP DC Voltage over", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "bat_open", "Bat open", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "current_sensor_fail", "Current sensor fail", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "battery_short", "Battery short", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "power_limit", "Power limit", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "pv_voltage_high", "PV Voltage high", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "mppt_overload_fault", "MPPT Overload fault", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "mppt_overload_warning", "MPPT Overload warning", None, "alert").await?;
    register_sensor(client, cfg, "qpiws", "battery_too_low_to_charge", "Battery too low to charge", None, "alert").await?;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    state_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_class: Option<String>,
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
    debug!("Registering error sensor");
    let params = SensorDiscoveryParams {
        unique_id: format!("{}_last_error", cfg.discovery.node_name).parse().unwrap(),
        name: format!("{} - Last error", cfg.discovery.device_name).to_string(),
        unit_of_measurement: None,
        value_template: None,
        state_topic: format!("{}/{}", cfg.topic, "error").to_string(),
        icon: "mdi:hammer-wrench".parse().unwrap(),
        device: get_device_hassio(&cfg),
        force_update: false,
        state_class: None,
        device_class: None,
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(format!("{}/sensor/{}/{}/config", cfg.discovery.prefix, cfg.discovery.node_name, "error").to_string(), params_string.as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(true);
    client.publish(&msg).await?;
    Ok(())
}

async fn register_sensor(client: &Client, cfg: &MqttSettings, command: &str, id: &str, name: &str, unit: Option<String>, icon: &str) -> Result<(), Box<dyn std::error::Error>> {
    let unique_id = format!("{}_{}_{}", cfg.discovery.node_name, command, id).to_string().replace(".", "_");
    let topic = format!("{}/{}", cfg.topic, command).to_string();

    debug!("Registering sensor {}", unique_id);
    let state_class = match unit {
        Some(_) => Some(String::from("measurement")),
        None => None,
    };
    // device_class and state_class enable long term statistics in home assistant
    let device_class = match unit {
        Some(ref unit) => match unit.as_str() {
            "Vac" | "Vdc" | "V" => Some(String::from("voltage")),
            "Aac" | "Adc" | "A" => Some(String::from("current")),
            "VA" => Some(String::from("apparent_power")),
            "%" => Some(String::from("battery")),
            "Wh" | "kWh" | "MWh" => Some(String::from("energy")),
            "Hz" | "kHz" | "MHz" | "GHz" => Some(String::from("frequency")),
            "W" | "kW" => Some(String::from("power")),
            "°C" | "°F" => Some(String::from("temperature")),
            _ => None,
        },
        None => None,
    };

    let params = SensorDiscoveryParams {
        unique_id,
        name: format!("{} - {}", cfg.discovery.device_name, name).to_string(),
        unit_of_measurement: unit,
        value_template: Some(format!("{{{{ value_json.{} }}}}", id).to_string()),
        state_topic: topic,
        icon: format!("mdi:{}", icon).to_string(),
        device: get_device_hassio(&cfg),
        force_update: false,
        state_class,
        device_class,
    };
    let params_string = serde_json::to_string(&params)?;
    let mut msg = PublishOpts::new(format!("{}/sensor/{}/{}_{}/config", cfg.discovery.prefix, cfg.discovery.node_name, command, id.replace(".", "_")).to_string(), params_string.as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(true);
    client.publish(&msg).await?;
    Ok(())
}
