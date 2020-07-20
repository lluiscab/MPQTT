use libc::{open, O_RDWR};
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::FromRawFd;
use std::path::Path;
use tokio::fs::File;

/// Open a new async `File` using the raw C `open` function.
///
/// Rust's `File::open` cannot be used since it most likely uses `fopen` which won't work with
/// actual streams, but only with real files.
///
/// Should be used when connecting to the inverter through the USB interface (not the serial one).
pub fn raw_open<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
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
