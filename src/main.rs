#![warn(clippy::all)]

mod mqtt_discovery;
mod settings;
use crate::mqtt_discovery::run_mqtt_discovery;
use crate::settings::MqttSettings;
use settings::Settings;

use masterpower_api::commands::qid::QID;
use masterpower_api::commands::qmod::QMOD;
use masterpower_api::commands::qpi::QPI;
use masterpower_api::commands::qpigs::QPIGS;
use masterpower_api::commands::qpiri::QPIRI;
use masterpower_api::commands::qpiws::QPIWS;
use masterpower_api::commands::qvfw::QVFW;
use masterpower_api::commands::qvfw2::QVFW2;
use masterpower_api::inverter::Inverter;

use libc::{open, O_RDWR};
use log::{debug, error, info};
use mqtt_async_client::client::{Client as MQTTClient, KeepAlive, Publish as PublishOpts, QoS};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use std::thread::sleep;
use std::time::Instant;
use tokio::fs::File;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting {} version {}", env!("CARGO_PKG_NAME").to_ascii_uppercase(), env!("CARGO_PKG_VERSION"));

    // Load configuration
    let settings = Settings::new();
    if let Err(e) = settings {
        println!("Error loading configuration file: {}", e);
        std::process::exit(1);
    }
    let settings = settings.unwrap();

    // Enable debugging
    if settings.debug {
        println!("Enabled debug output");
        std::env::set_var("RUST_LOG", "error,mpqtt=trace,masterpower_api=trace");
        pretty_env_logger::init();
    }

    // Create MQTT Connection
    info!("Connecting to MQTT Broker at: {}:{}", settings.mqtt.host, settings.mqtt.port);
    let mut builder = mqtt_async_client::client::Client::builder();
    let mut mqtt_client = builder
        .set_host(settings.mqtt.host.clone())
        .set_port(settings.mqtt.port)
        .set_username(Option::from(settings.mqtt.username.clone()))
        .set_password(Option::from(settings.mqtt.password.as_bytes().to_vec()))
        .set_client_id(Option::from(settings.mqtt.client_id.clone()))
        .set_connect_retry_delay(Duration::from_secs(1))
        .set_keep_alive(KeepAlive::from_secs(5))
        .set_operation_timeout(Duration::from_secs(5))
        .set_automatic_connect(true)
        .build()?;

    mqtt_client.connect().await?;
    info!("Connected to MQTT Broker");

    // Run MQTT Discovery
    run_mqtt_discovery(&mqtt_client, &settings.mqtt).await?;

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

    // Start
    let init_res = init(&mut inverter, &mqtt_client, &settings).await;
    if let Err(error) = init_res {
        publish_error(&mqtt_client, &settings.mqtt, error.to_string()).await?;
        error!("{}", error);
        std::process::exit(1);
    }

    // Update loop
    loop {
        // Do update
        let upd = update(&mut inverter, &mqtt_client, &settings).await;
        if let Err(error) = upd {
            publish_error(&mqtt_client, &settings.mqtt, error.to_string()).await?;
            error!("{}", error);
        } else {
            clear_error(&mqtt_client, &settings.mqtt).await?;
        }

        // Sleep 1 sec
        sleep(Duration::from_secs(1));
    }
}

async fn init(inverter: &mut Inverter<File>, mqtt_client: &MQTTClient, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    // Get initial values

    // QID      - Serial number
    let serial_number = inverter.execute::<QID>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qid", serde_json::to_string(&serial_number)?).await?;

    // QPI      - Protocol ID
    let protocol_id = inverter.execute::<QPI>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qpi", serde_json::to_string(&protocol_id)?).await?;

    // QVFW     - Software version 1
    let software_version_1 = inverter.execute::<QVFW>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qvfw", serde_json::to_string(&software_version_1)?).await?;

    // QVFW2     - Software version 2
    let software_version_2 = inverter.execute::<QVFW2>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qvfw2", serde_json::to_string(&software_version_2)?).await?;

    Ok(())
}

async fn update(inverter: &mut Inverter<File>, mqtt_client: &MQTTClient, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    // Start update
    debug!("Starting update");
    let start = Instant::now();

    // QMOD     -  Device Mode Inquiry
    let qmod = inverter.execute::<QMOD>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qmod", serde_json::to_string(&qmod)?).await?;

    // QPIRI    - Device Rating Information Inquiry
    let qpiri = inverter.execute::<QPIRI>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qpiri", serde_json::to_string(&qpiri)?).await?;

    // QPIGS    - Device general status parameters inquiry
    let qpigs = inverter.execute::<QPIGS>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qpigs", serde_json::to_string(&qpigs)?).await?;

    // QPIWS    - Device Warning Status Inquiry
    let qpiws = inverter.execute::<QPIWS>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qpiws", serde_json::to_string(&qpiws)?).await?;

    // Report update completed
    debug!("Update finished without errors");
    let time = start.elapsed().as_millis();
    info!("Update took {}ms", time);

    Ok(())
}

async fn publish_update(mqtt_client: &MQTTClient, mqtt: &MqttSettings, command: &str, value: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = PublishOpts::new(format!("{}/{}", mqtt.topic, command).to_string(), Vec::from(value));
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    mqtt_client.publish(&msg).await?;
    Ok(())
}

async fn publish_error(mqtt_client: &MQTTClient, mqtt: &MqttSettings, error: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = PublishOpts::new(format!("{}/error", mqtt.topic).to_string(), Vec::from(error));
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    mqtt_client.publish(&msg).await?;
    Ok(())
}

async fn clear_error(mqtt_client: &MQTTClient, mqtt: &MqttSettings) -> Result<(), Box<dyn std::error::Error>> {
    let mut msg = PublishOpts::new(format!("{}/error", mqtt.topic).to_string(), "".to_string().as_bytes().to_vec());
    msg.set_qos(QoS::AtLeastOnce);
    msg.set_retain(false);
    mqtt_client.publish(&msg).await?;
    Ok(())
}

fn raw_open<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    let fd = unsafe { open(path.as_ref().as_os_str().as_bytes().as_ptr() as *const i8, O_RDWR) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let std_file = unsafe { std::fs::File::from_raw_fd(fd) };
    Ok(File::from_std(std_file))
}
