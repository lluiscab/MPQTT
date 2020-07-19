use crate::inverter::Inverter;
use tokio::time::Duration;
use tokio::fs::File;
use std::path::Path;
use std::ffi::{CStr, CString};

mod crc;
mod inverter;

#[tokio::main(core_threads=1, max_threads=1)]
async fn main() {
    // let mut inverter = Inverter::new(
    //     Serial::from_path(
    //         "/dev/hidraw2",
    //         &SerialPortSettings {
    //             baud_rate: 2400,
    //             // WTF is start bit? <--
    //             data_bits: DataBits::Eight,
    //             // flow_control: FlowControl::None,
    //             parity: Parity::None,
    //             stop_bits: StopBits::One,
    //             timeout: Duration::from_secs(2),
    //             ..Default::default()
    //         },
    //     )
    //         .unwrap(),
    // );
    //let file = File::open("/dev/hidraw2").await.unwrap();
    let file = raw_open("/dev/hidraw2").unwrap();
    let mut inverter = Inverter::new(file);

    inverter.query_serial_number().await.unwrap();

    // Start

    // QID      - Serial number
    // QVFW     - CPU Firmware version
    // QVFW2    - CPU Firmware version 2


    // Loop

    // QMOD     -  Device Mode Inquiry
    // QPIGS    - Device general status parameters inquiry
    // QPIRI    - Device Rating Information Inquiry
    // QPIWS    - Device Warning Status Inquiry


}

fn raw_open<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    use libc::{open, O_RDWR};
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::io::FromRawFd;

    let fd = unsafe { open(CString::new(path.as_ref().as_os_str().as_bytes()).unwrap().as_ptr(), O_RDWR) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }

    // This is where tcgetattr and tcsetattr would go. HOWEVER, they FOR SOME REASON WON'T WORK!!!

    let mut std_file = unsafe { std::fs::File::from_raw_fd(fd) };
    Ok(File::from_std(std_file))
}
