#![warn(clippy::all)]

mod mqtt_discovery;
mod settings;
use crate::mqtt_discovery::run_mqtt_discovery;
use crate::settings::MqttSettings;
use settings::Settings;

use masterpower_api::commands::qid::QID;
use masterpower_api::commands::qmod::QMOD;
use masterpower_api::commands::qpgs::{QPGS0, QPGS1, QPGS2};
use masterpower_api::commands::qpi::QPI;
use masterpower_api::commands::qpigs::QPIGS;
use masterpower_api::commands::qpiri::QPIRI;
use masterpower_api::commands::qpiws::QPIWS;
use masterpower_api::commands::qvfw::QVFW;
// use masterpower_api::commands::qvfw2::QVFW2;
// use masterpower_api::commands::qvfw3::QVFW3;
use masterpower_api::inverter::Inverter;

use libc::{open, O_RDWR};
use log::{debug, error, info};
use mqtt_async_client::client::{Client as MQTTClient, KeepAlive, Publish as PublishOpts, QoS};
use serde_derive::Serialize;
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
    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            println!("Error loading configuration file: {}", e);
            std::process::exit(1);
        }
    };

    // Enable debugging
    if settings.debug {
        std::env::set_var("RUST_LOG", "warn,mpqtt=trace,masterpower_api=trace");
        info!("Enabled debug output");
    } else {
        std::env::set_var("RUST_LOG", "error,mpqtt=info,masterpower_api=info");
    }
    pretty_env_logger::init();

    // Create MQTT Connection
    info!("Connecting to MQTT Broker at: {}:{}", settings.mqtt.host, settings.mqtt.port);
    let mut builder = mqtt_async_client::client::Client::builder();
    let mut mqtt_client = match builder
        .set_host(settings.mqtt.host.clone())
        .set_port(settings.mqtt.port)
        .set_username(Option::from(settings.mqtt.username.clone()))
        .set_password(Option::from(settings.mqtt.password.as_bytes().to_vec()))
        .set_client_id(Option::from(settings.mqtt.client_id.clone()))
        .set_connect_retry_delay(Duration::from_secs(1))
        .set_keep_alive(KeepAlive::from_secs(5))
        .set_operation_timeout(Duration::from_secs(5))
        .set_automatic_connect(true)
        .build()
    {
        Ok(val) => val,
        Err(err) => {
            error!("Problem with MQTT client builder: {}", err);
            std::process::exit(0);
        }
    };

    mqtt_client.connect().await?;
    info!("Connected to MQTT Broker");

    // Run MQTT Discovery
    run_mqtt_discovery(&mqtt_client, &settings.mqtt).await?;

    // Open inverter tty device - wrap open call in for loop with timeout and a break on success
    let stream = match raw_open(settings.inverter.path.clone()) {
        Ok(stream) => stream,
        Err(err) => {
            // Handle error opening inverter
            publish_error(&mqtt_client, &settings.mqtt, err.to_string()).await?; // wrap in loop to retry publish on fails
            error!("Could not open inverter communication {}", err);
            todo!("implement retrying on file not found or couldn't open with warn! before error!");
        }
    };

    // Clear previous errors
    clear_error(&mqtt_client, &settings.mqtt).await?; // wrap in loop to retry publish on fails

    // Create inverter instance
    let mut inverter = Inverter::from_stream(stream);

    // Start
    let init_res = init(&mut inverter, &mqtt_client, &settings).await;
    if let Err(error) = init_res {
        publish_error(&mqtt_client, &settings.mqtt, error.to_string()).await?;
        error!("Error initialising inverter: {}", error);
        todo!("implement retrying on file not found or couldn't open with warn! before error!");
        // std::process::exit(1);
    }

    // Update loop
    loop {
        let upd = update(&mut inverter, &mqtt_client, &settings).await;
        if let Err(error) = upd {
            publish_error(&mqtt_client, &settings.mqtt, error.to_string()).await?;
            error!("Error publishing error: {}", error);
        } else {
            clear_error(&mqtt_client, &settings.mqtt).await?;
        }
    }
}

async fn init(inverter: &mut Inverter<File>, mqtt_client: &MQTTClient, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    // Get initial values

    // QID      - Serial number
    match inverter.execute::<QID>(()).await {
        Ok(serial_number) => {
            publish_update(&mqtt_client, &settings.mqtt, "qid", serde_json::to_string(&serial_number)?).await?;
        }
        Err(serial_number_error) => {
            error!("Error fetching serial number: {}", serial_number_error);
        }
    };
    // QPI      - Protocol ID
    let protocol_id = inverter.execute::<QPI>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qpi", serde_json::to_string(&protocol_id)?).await?;

    // QVFW     - Software version 1
    let software_version_1 = inverter.execute::<QVFW>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qvfw", serde_json::to_string(&software_version_1)?).await?;

    debug!("Completed init commands");

    Ok(())
}

async fn update(inverter: &mut Inverter<File>, mqtt_client: &MQTTClient, settings: &Settings) -> Result<(), Box<dyn std::error::Error>> {
    // Start update
    debug!("Starting new update");
    let outer_start = Instant::now();
    let mut inner_time: u128 = 0;
    for _ in 0..settings.inner_iterations {
        let inner_start = Instant::now();
        if settings.debug {
            // spare the connection some bandwidth in general use
            let qpgs0 = inverter.execute::<QPGS0>(()).await?;
            publish_update(&mqtt_client, &settings.mqtt, "qpgs0", serde_json::to_string(&qpgs0)?).await?;
        }
        // main update loop for phocos
        let qpgs1 = inverter.execute::<QPGS1>(()).await?;
        let qpgs2 = inverter.execute::<QPGS2>(()).await?;
        publish_update(&mqtt_client, &settings.mqtt, "qpgs1", serde_json::to_string(&qpgs1)?).await?;
        publish_update(&mqtt_client, &settings.mqtt, "qpgs2", serde_json::to_string(&qpgs2)?).await?;
        // QPIGS    - Device general status parameters inquiry
        if settings.mode != String::from("phocos") {
            let qpigs = inverter.execute::<QPIGS>(()).await?;
            publish_update(&mqtt_client, &settings.mqtt, "qpigs", serde_json::to_string(&qpigs)?).await?;
        }
        inner_time = inner_start.elapsed().as_millis(); // calculate average for this for the stats sensor
        info!("Partial update took {}ms", inner_time);
        sleep(Duration::from_secs(settings.inner_delay));
    }

    // QMOD     -  Device Mode Inquiry
    let qmod = inverter.execute::<QMOD>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qmod", serde_json::to_string(&qmod)?).await?;
    // QPIRI    - Device Rating Information Inquiry
    if settings.mode != String::from("phocos") {
        // I think it could be implemented for phocos, just needs some work
        let qpiri = inverter.execute::<QPIRI>(()).await?;
        publish_update(&mqtt_client, &settings.mqtt, "qpiri", serde_json::to_string(&qpiri)?).await?;
    }
    // QPIWS    - Device Warning Status Inquiry
    let qpiws = inverter.execute::<QPIWS>(()).await?;
    publish_update(&mqtt_client, &settings.mqtt, "qpiws", serde_json::to_string(&qpiws)?).await?;

    // Report update completed
    let outer_time = outer_start.elapsed().as_millis();
    info!("Full update took {}ms", outer_time);
    let stats = StatsSensor {
        outer_update_duration: outer_time,
        inner_update_duration: inner_time,
    };
    publish_update(&mqtt_client, &settings.mqtt, "stats", serde_json::to_string(&stats)?).await?;
    sleep(Duration::from_secs(settings.outer_delay));
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
    let fd = unsafe { open(path.as_ref().as_os_str().as_bytes().as_ptr() as *const u8, O_RDWR) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    let std_file = unsafe { std::fs::File::from_raw_fd(fd) };
    Ok(File::from_std(std_file))
}

#[derive(Serialize, Debug)]
struct StatsSensor {
    outer_update_duration: u128,
    inner_update_duration: u128,
}
