use mecha_display::{Display, DisplayInterface};
use mecha_power_supply::{Battery, PowerSupplyInfo};

use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};

pub struct PowerTest2 {
    pub display_path: String,
    pub camera_path: String,
    pub current_now: String,
}

impl TestAssertion for PowerTest2 {
    fn describe(&self) -> &str {
        "Power Test 2"
    }

    fn test(&self) -> anyhow::Result<bool> {
        log_message(Device::Power, MessageType::Test, "Power Test 2 Basic");

        log_message(Device::Power, MessageType::Info, "Printing Test Conditions");

        log_message(
            Device::Power,
            MessageType::Action,
            "Setting Display Brightness to 80%",
        );

        let mut display = Display {
            path: String::new(),
        };

        // Set the display path.
        display.set_device(&self.display_path);

        // Check if the display path is empty after calling set_device.
        if display.path.is_empty() {
            log_message(
                Device::Power,
                MessageType::Error,
                &format!("Unable to find display"),
            );
            return Ok(false);
        }

        if let Err(err) = display.set_brightness(204) {
            log_message(
                Device::Power,
                MessageType::Error,
                &format!("Failed to set display brightness: {}", err),
            );
            return Ok(false);
        }

        log_message(Device::Power, MessageType::Info, "Camera - Off");
        log_message(Device::Power, MessageType::Info, "Audio - Off");

        // Ask user whether the battery is charging or not.
        let user_response = question_prompt(
            Device::Power,
            MessageType::Confirm,
            "Is the battery charging?".to_owned(),
        );

        if user_response {
            log_message(Device::Power, MessageType::Info, "Battery - Charging");
        } else {
            log_message(Device::Power, MessageType::Info, "Battery - Not Charging");
        }

        //wait for 15 seconds for user to check the power consumption in the mean time log info message that wait for 15 seconds
        log_message(
            Device::Power,
            MessageType::Action,
            "Waiting for 15 seconds to check power consumption",
        );

        std::thread::sleep(std::time::Duration::from_secs(15));

        let battery = Battery {
            path: String::new(),
            currnet_now: self.current_now.clone(),
        };

        let currnet = battery.get_current()?;

        // the current value will be six digit number we need to convert it to mA
        let current = currnet / 1000;

        // Print the value for current from the battery.
        log_message(
            Device::Power,
            MessageType::Info,
            &format!("Current: {} mA", current),
        );
        Ok(true)
    }
}
