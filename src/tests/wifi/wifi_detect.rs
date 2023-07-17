use crate::test_base::{log_message, Device, MessageType, TestAssertion};

pub struct WifiDetectionTest;

//imple testAssertion for WifiDetectionTest
impl TestAssertion for WifiDetectionTest {
    fn describe(&self) -> &str {
        "Wifi Detection Test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let wifi = "wifi is avilabe";

        log_message(Device::WiFi, MessageType::Info, "Detecting Wifi");

        // if wifi info is empty, then the test failed else it passed on fail log the error
        if wifi.is_empty() {
            log_message(
                Device::WiFi,
                MessageType::Error,
                "Wifi not detected, please check the connection",
            );
            Ok(false)
        } else {
            log_message(Device::WiFi, MessageType::Pass, "Wifi detected");
            Ok(true)
        }
    }
}
