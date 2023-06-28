use std::io;
use std::process::Command;

// Define the TrustIcData trait
pub trait TrustIcData {
    fn chip_info(&self) -> Result<String, io::Error>;
}

// Implement the TrustIcData trait for the TrustIc struct
pub struct TrustIc;

impl TrustIcData for TrustIc {
    fn chip_info(&self) -> Result<String, io::Error> {
        let output = Command::new("/MECHA_TEST/optiga_trust_m/trustm_chipinfo").output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.into_owned())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Command executed with error: {}", stderr),
            ))
        }
    }
}
