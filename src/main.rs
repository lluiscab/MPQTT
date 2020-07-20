use crate::inverter::util::raw_open;
use crate::inverter::Inverter;

mod crc;
mod inverter;

// Params.
const TTY_DEVICE: &'static str = "/dev/hidraw2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inverter = Inverter::from_stream(raw_open(TTY_DEVICE)?);

    println!("Serial number: {}", inverter.query_serial_number().await?);

    // Start

    // QID      - Serial number
    // QVFW     - CPU Firmware version
    // QVFW2    - CPU Firmware version 2

    // Loop

    // QMOD     -  Device Mode Inquiry
    // QPIGS    - Device general status parameters inquiry
    // QPIRI    - Device Rating Information Inquiry
    // QPIWS    - Device Warning Status Inquiry

    Ok(())
}
