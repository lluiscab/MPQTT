use log::{error, info};

use mqtt_async_client::client::{KeepAlive, Publish as PublishOpts, QoS};

use libc::{open, O_RDWR};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use tokio::fs::File;
mod response_serialize;
mod settings;

use settings::Settings;

use crate::response_serialize::JSONQPIGSResponse;
use masterpower_api::commands::qpigs::DeviceChargingStatus::ChargingFromSCCAndAC;
use masterpower_api::commands::qpigs::{DeviceStatus, QPIGSResponse, QPIGS};
use masterpower_api::inverter::Inverter;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let settings = Settings::new();
    if let Err(e) = settings {
        println!("Error loading configuration file: {}", e);
        std::process::exit(1);
    }
    let settings = settings.unwrap();

    // Enable debugging
    if settings.debug {
        std::env::set_var("RUST_LOG", "trace");
        pretty_env_logger::init();
    }

    // Create MQTT Connection
    info!(
        "Connecting to MQTT Broker at: {}:{}",
        settings.mqtt.host, settings.mqtt.port
    );
    let mut builder = mqtt_async_client::client::Client::builder();
    let mut mqtt_client = builder
        .set_host(settings.mqtt.host)
        .set_port(settings.mqtt.port)
        .set_username(Option::from(settings.mqtt.username))
        .set_password(Option::from(
            settings.mqtt.password.to_string().as_bytes().to_vec(),
        ))
        .set_client_id(Option::from(settings.mqtt.client_id))
        .set_connect_retry_delay(Duration::from_secs(1))
        .set_keep_alive(KeepAlive::from_secs(5))
        .set_operation_timeout(Duration::from_secs(5))
        .set_automatic_connect(true)
        .build()?;

    mqtt_client.connect().await?;
    info!("Connected to MQTT Broker");

    // Open inverter tty device
    let stream = raw_open(settings.inverter.path);

    let json_qpigs: JSONQPIGSResponse = QPIGSResponse {
        grid_voltage: 1.0f32,
        grid_frequency: 0.0f32,
        ac_out_voltage: 229.0f32,
        ac_out_frequency: 50.0f32,
        ac_out_apparent_power: 91,
        ac_out_active_power: 91,
        out_load_percent: 3,
        bus_voltage: 420,
        battery_voltage: 27.16f32,
        battery_charge_current: 0,
        battery_capacity: 100,
        inverter_heat_sink_temp: 336,
        pv_input_current: 0,
        pv_input_voltage: 74.9f32,
        battery_scc_voltage: 27.12f32,
        battery_discharge_current: 5,
        device_status: DeviceStatus {
            active_load: true,
            charge_status: ChargingFromSCCAndAC,
        },
    }
    .into();
    println!("{}", serde_json::to_string(&json_qpigs)?);

    // Handle inverter error
    if let Err(error) = stream {
        let mut msg = PublishOpts::new(
            format!("{}/error", settings.mqtt.topic).to_string(),
            error.to_string().as_bytes().to_vec(),
        );
        msg.set_qos(QoS::AtLeastOnce);
        msg.set_retain(false);
        mqtt_client.publish(&msg).await?;

        error!("Could not open inverter communication");
        eprintln!("{}", error);
        std::process::exit(1);
    }

    // Create inverter instance
    let mut inverter = Inverter::from_stream(stream.unwrap());

    // Start

    // QID      - Serial number
    // let serial_number = inverter.execute::<QID>(()).await?.0;
    // println!("Serial number: {}", serial_number);

    // QPI      - Protocol ID
    // let protocol_id = inverter.execute::<QPI>(()).await?.0;
    // println!("Protocol ID: {}", protocol_id);

    // QVFW     - CPU Firmware version
    // let cpu_firmware = inverter.execute::<QVFW>(()).await?.0;
    // println!("CPU Firmware version: {}", cpu_firmware);

    // QVFW2    - CPU Firmware version 2
    // let cpu_firmware2 = inverter.execute::<QVFW2>(()).await?.0;
    // println!("CPU Firmware version 2: {}", cpu_firmware2);

    // Loop
    loop {
        // QMOD     -  Device Mode Inquiry

        // QPIGS    - Device general status parameters inquiry
        let qpigs = inverter.execute::<QPIGS>(()).await?;
        let json_qpigs: JSONQPIGSResponse = qpigs.into();

        let mut msg = PublishOpts::new(
            format!("{}/qpigs", settings.mqtt.topic).to_string(),
            serde_json::to_string(&json_qpigs)?.as_bytes().to_vec(),
        );
        msg.set_qos(QoS::AtLeastOnce);
        msg.set_retain(false);
        mqtt_client.publish(&msg).await?;

        // QPIRI    - Device Rating Information Inquiry
        // QPIWS    - Device Warning Status Inquiry
    }
}

fn raw_open<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    let fd = unsafe {
        open(
            CString::new(path.as_ref().as_os_str().as_bytes())
                .unwrap()
                .as_ptr(),
            O_RDWR,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let std_file = unsafe { std::fs::File::from_raw_fd(fd) };
    Ok(File::from_std(std_file))
}
