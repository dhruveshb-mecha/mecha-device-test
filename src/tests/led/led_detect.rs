use mecha_led::{Led, LedColor, LedInterface};

use crate::test_base::{log_message, Device, MessageType, TestAssertion};

pub struct LedDetect {
    pub red: String,
    pub green: String,
    pub blue: String,
}

impl TestAssertion for LedDetect {
    fn describe(&self) -> &str {
        "LED Detection test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let mut led = Led {
            red: self.red.clone(),
            green: self.green.clone(),
            blue: self.blue.clone(),
        };

        log_message(Device::Led, MessageType::Test, "Led Detection Test");
        log_message(
            Device::Led,
            MessageType::Action,
            "Setting all LED values to zero",
        );
        // Set all LED values to zero

        //if we can not set led values then log message as fail or else set led values
        if led.set_device(LedColor::Red, 0).is_err()
            || led.set_device(LedColor::Green, 0).is_err()
            || led.set_device(LedColor::Blue, 0).is_err()
        {
            log_message(Device::Led, MessageType::Fail, "Unable to set LED values");
            return Ok(false);
        } else {
            log_message(Device::Led, MessageType::Pass, "LED values set to zero");
            led.set_device(LedColor::Red, 0)?;
            led.set_device(LedColor::Green, 0)?;
            led.set_device(LedColor::Blue, 0)?;

            // Get LED values and verify if they are all zero
            let (red_value, green_value, blue_value) = led.info()?;
            if red_value == 0 && green_value == 0 && blue_value == 0 {
                Ok(true) // Test passed
            } else {
                Ok(false) // Test failed
            }
        }
    }
}
