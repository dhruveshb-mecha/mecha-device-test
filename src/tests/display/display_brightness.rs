use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};
use anyhow::Result;
use mecha_display::{Display, DisplayInterface};

pub(crate) struct DisplayBrightness {
    pub device: String,
}

impl TestAssertion for DisplayBrightness {
    fn describe(&self) -> &str {
        "Display Detect"
    }

    fn test(&self) -> Result<bool> {
        let mut display = Display {
            path: String::new(),
        };

        //log action that setting display device
        log_message(
            Device::Display,
            MessageType::Action,
            "Setting Display Device",
        );

        // will be replaced with actual behavior using SDK
        display.set_device("/sys/class/backlight/backlight/brightness");

        // Brightness level to test (10% and 100%)
        let brightness_levels = [16, 255];

        log_message(
            Device::Display,
            MessageType::Action,
            "Setting Display Brightness",
        );

        for brightness in &brightness_levels {
            if let Err(err) = display.set_brightness(*brightness) {
                let error_message = format!("Failed to set display brightness: {}", err);
                log_message(Device::Display, MessageType::Error, &error_message);
                return Ok(false);
            }

            let user_response = question_prompt(
                Device::Display,
                MessageType::Confirm,
                format!("is the display brightness set to {}%?", brightness),
            );
            if user_response {
                let user_rsepose_message = format!("Display Brightness: {}", brightness);
                log_message(Device::Display, MessageType::Pass, &user_rsepose_message);
            } else {
                let current_brightness = display.get_brightness()?;
                let user_resonse_message = format!("Display Brightness: {}", current_brightness);
                log_message(Device::Display, MessageType::Fail, &user_resonse_message);
                return Ok(false);
            }
        }

        Ok(true)
    }
}
