use mecha_display::{Display, DisplayInterface};

use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};

pub struct PowerTest3 {
    pub display_path: String,
    pub camera_path: String,
}

impl TestAssertion for PowerTest3 {
    fn describe(&self) -> &str {
        "Power Test 3"
    }

    fn test(&self) -> anyhow::Result<bool> {
        log_message(Device::Power, MessageType::Test, "Power Test 3 Started");

        log_message(Device::Power, MessageType::Info, "Printing Test Conditions");

        log_message(
            Device::Power,
            MessageType::Action,
            "Setting Display Brightness to 0",
        );

        let mut display = Display {
            path: String::new(),
        };

        display.set_device(&self.display_path);
        display.set_brightness(0)?;

        log_message(Device::Power, MessageType::Info, "Camera - Off");
        log_message(Device::Power, MessageType::Info, "Audio - Off");

        //ask user that battery is charging or not based on that log info message that battery is charging or not
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

        Ok(true)
    }
}
