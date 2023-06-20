use crate::test_base::{log_message, Device, MessageType, TestAssertion};
use anyhow::Result;

pub(crate) struct DisplayDetect {
    pub device: String,
}

impl TestAssertion for DisplayDetect {
    fn describe(&self) -> &str {
        "Display Detection test"
    }

    fn test(&self) -> Result<bool> {
        let display_detected = true; // Replace with your actual detection logic
        log_message(Device::Display, MessageType::Action, "Detecting Display");
        if display_detected {
            log_message(Device::Display, MessageType::Info, "Display Detected");
            Ok(true)
        } else {
            log_message(
                Device::Display,
                MessageType::Fail,
                "Display Detection test failed!",
            );
            Ok(false) // Return false to indicate test failure, but still continue executing other tests
        }
    }
}
