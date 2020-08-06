use libc::{open, O_RDWR};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use tokio::fs::File;

use masterpower_api::inverter::Inverter;
use masterpower_api::commands::qid::{QID};
use masterpower_api::commands::qpi::{QPI};
use masterpower_api::commands::qvfw::{QVFW};
use masterpower_api::commands::qvfw2::{QVFW2};
use masterpower_api::commands::qpigs::{QPIGS};

// Params.
const TTY_DEVICE: &'static str = "/dev/inverter";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Enable debugging
    std::env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    // Create connection and inverter instance
    let mut inverter = Inverter::from_stream(raw_open(TTY_DEVICE)?);

    // Start

    // QID      - Serial number
    // let serial_number = inverter.execute::<QID>(()).await?.0;
    // println!("Serial number: {}", serial_number);
    
    // QPI      - Protocol ID
    // let protocol_id = inverter.execute::<QPI>(()).await?.0;
    // println!("Protocol ID: {}", protocol_id);

    // QVFW     - CPU Firmware version
    let cpu_firmware = inverter.execute::<QVFW>(()).await?.0;
    println!("CPU Firmware version: {}", cpu_firmware);

    // QVFW2    - CPU Firmware version 2
    let cpu_firmware2 = inverter.execute::<QVFW2>(()).await?.0;
    println!("CPU Firmware version 2: {}", cpu_firmware2);

    // Loop

    // QPIGS
    let qpigs =inverter.execute::<QPIGS>(()).await?.0;
    println!("QPIGS: {}", qpigs);



    // Loop

    // QMOD     -  Device Mode Inquiry
    // QPIGS    - Device general status parameters inquiry
    // QPIRI    - Device Rating Information Inquiry
    // QPIWS    - Device Warning Status Inquiry

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

    // This is where tcgetattr and tcsetattr would go. HOWEVER, they FOR SOME REASON WON'T WORK!!!

    let std_file = unsafe { std::fs::File::from_raw_fd(fd) };
    Ok(File::from_std(std_file))
}
