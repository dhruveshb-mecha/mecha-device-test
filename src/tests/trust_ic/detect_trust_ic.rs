use mecha_trust_ic::{TrustIc, TrustIcData};

use crate::test_base::{log_message, Device, MessageType, TestAssertion};

pub struct TrustIcDetectionTest;

impl TestAssertion for TrustIcDetectionTest {
    fn describe(&self) -> &str {
        "Trust IC Detection Test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let trust_ic = TrustIc;
        let chip_info = trust_ic.chip_info()?;

        // if chip info is empty, then the test failed else it passed on fail log the error
        if chip_info.is_empty() {
            log_message(
                Device::TrustIc,
                MessageType::Error,
                "Trust IC not detected, please check the connection",
            );
            Ok(false)
        } else {
            log_message(Device::TrustIc, MessageType::Info, "Trust IC detected");
            Ok(true)
        }
    }
}
