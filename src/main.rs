mod mqtt_discovery;
mod response_serialize;
mod settings;
use crate::mqtt_discovery::run_mqtt_discovery;
use crate::response_serialize::{JSONQPIGSResponse, JSONQIDResponse};
use settings::Settings;

use masterpower_api::commands::qpigs::QPIGS;
use masterpower_api::inverter::Inverter;

use crate::settings::MqttSettings;
use libc::{open, O_RDWR};
use log::{error, info};
use mqtt_async_client::client::{Client as MQTTClient, KeepAlive, Publish as PublishOpts, QoS};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use tokio::fs::File;
use tokio::time::Duration;
use std::thread::sleep;
use masterpower_api::commands::qid::QID;

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
        std::env::set_var("RUST_LOG", "error,inverter=trace,masterpower_api=trace");
        pretty_env_logger::init();
    }
    // Create MQTT Connection
    info!(
        "Connecting to MQTT Broker at: {}:{}",
        settings.mqtt.host, settings.mqtt.port
    );
    let mut builder = mqtt_async_client::client::Client::builder();
    let mut mqtt_client = builder
        .set_host(settings.mqtt.host.clone())
        .set_port(settings.mqtt.port)
        .set_username(Option::from(settings.mqtt.username.clone()))
        .set_password(Option::from(
            settings.mqtt.password.to_string().as_bytes().to_vec(),
        ))
        .set_client_id(Option::from(settings.mqtt.client_id.clone()))
        .set_connect_retry_delay(Duration::from_secs(1))
        .set_keep_alive(KeepAlive::from_secs(5))
        .set_operation_timeout(Duration::from_secs(5))
        .set_automatic_connect(true)
        .build()?;

    mqtt_client.connect().await?;
    info!("Connected to MQTT Broker");

    // Run MQTT Discovery if enabled
    if settings.mqtt.discovery.enabled {
        run_mqtt_discovery(&mqtt_client, &settings.mqtt).await?;
    }

    // Open inverter tty device
    let stream = raw_open(settings.inverter.path.clone());

    // Handle inverter error
    if let Err(error) = stream {
        publish_error(&mqtt_client, &settings.mqtt, error.to_string()).await?;
        error!("Could not open inverter communication {}", error);
        std::process::exit(1);
    }

    // Clear previous errors
    clear_error(&mqtt_client, &settings.mqtt).await?;

    // Create inverter instance
    let mut inverter = Inverter::from_stream(stream.unwrap());

    // TODO: Also loop this?
    // Start

    // QID      - Serial number
    // let serial_number = inverter.execute::<QID>(()).await?.0;
    // let json_qid = JSONQIDResponse {
    //     serial_number: serial_number.to_string()
    // };
    // publish_update(&mqtt_client, &settings.mqtt, "qid", serde_json::to_string(&json_qid)?).await?;

    // QPI      - Protocol ID
    // let protocol_id = inverter.execute::<QPI>(()).await?.0;
    // println!("Protocol ID: {}", protocol_id);

    // Update loop
    loop {

        // Do update
        let upd = update(&mut inverter, &mqtt_client, &settings).await;
        if let Err(error) = upd {
            publish_error(&mqtt_client, &settings.mqtt, error.to_string()).await?;
            error!("{}", error);
        }
        else {
            clear_error(&mqtt_client, &settings.mqtt).await?;
        }

        // Sleep 1 sec
        sleep(Duration::from_secs(1));
    }
}

async fn update(
    inverter: &mut Inverter<File>,
    mqtt_client: &MQTTClient,
    settings: &Settings,
) -> Result<(), Box<dyn std::error::Error>> {
    // QMOD     -  Device Mode Inquiry

    // QPIGS    - Device general status parameters inquiry
    let qpigs = inverter.execute::<QPIGS>(()).await?;
    let json_qpigs: JSONQPIGSResponse = qpigs.into();
    publish_update(&mqtt_client, &settings.mqtt, "qpigs", serde_json::to_string(&json_qpigs)?).await?;

    // QPIRI    - Device Rating Information Inquiry
    // QPIWS    - Device Warning Status Inquiry

    Ok(())
}

async fn publish_update(
    mqtt_client: &MQTTClient,
    mqtt: &MqttSettings,
    command: &str,
    value: String
) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = PublishOpts::new(
        format!("{}/{}", mqtt.topic, command).to_string(),
        Vec::from(value)
    );
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    mqtt_client.publish(&msg).await?;
    Ok(())
}

async fn publish_error(
    mqtt_client: &MQTTClient,
    mqtt: &MqttSettings,
    error: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = PublishOpts::new(
        format!("{}/error", mqtt.topic).to_string(),
        Vec::from(error)
    );
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    mqtt_client.publish(&msg).await?;
    Ok(())
}

async fn clear_error(
    mqtt_client: &MQTTClient,
    mqtt: &MqttSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = PublishOpts::new(
        format!("{}/error", mqtt.topic).to_string(),
        "".to_string().as_bytes().to_vec(),
    );
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    mqtt_client.publish(&msg).await?;
    Ok(())
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
