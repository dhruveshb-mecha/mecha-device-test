use mecha_led::{Led, LedColor, LedInterface};

use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};

pub struct LedColorTest {
    pub red: String,
    pub green: String,
    pub blue: String,
}

impl TestAssertion for LedColorTest {
    fn describe(&self) -> &str {
        "LED Detection test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let mut led = Led {
            red: self.red.clone(),
            green: self.green.clone(),
            blue: self.blue.clone(),
        };

        // Set Red LED to 0 and Green/Blue LEDs to 1 if we can set device or else log message as fail
        if led.set_device(LedColor::Red, 0).is_err()
            || led.set_device(LedColor::Green, 1).is_err()
            || led.set_device(LedColor::Blue, 1).is_err()
        {
            log_message(
                Device::Led,
                MessageType::Fail,
                "Unable to set Red LED values",
            );
            return Ok(false); // Test failed
        } else {
            // Confirm if the user can see the Red LED color (which should be off)
            if !question_prompt(
                Device::Led,
                MessageType::Confirm,
                "Can you see the Red LED color?".to_string(),
            ) {
                log_message(Device::Led, MessageType::Fail, "LED test failed");
                return Ok(false); // Test failed
            } else {
                log_message(Device::Led, MessageType::Pass, "LED test passed");
            }
        }

        // Set Red LED to 1 and Green/Blue LEDs to 0 if we can set device or else log message as fail
        if led.set_device(LedColor::Red, 0).is_err()
            || led.set_device(LedColor::Green, 1).is_err()
            || led.set_device(LedColor::Blue, 0).is_err()
        {
            log_message(
                Device::Led,
                MessageType::Fail,
                "Unable to set Green LED values",
            );
            return Ok(false); // Test failed
        } else {
            // Confirm if the user can see the Red LED color (which should be off)
            if !question_prompt(
                Device::Led,
                MessageType::Confirm,
                "Can you see the Green LED color?".to_string(),
            ) {
                log_message(Device::Led, MessageType::Fail, "Green LED test failed");
                return Ok(false); // Test failed
            } else {
                log_message(Device::Led, MessageType::Pass, "Green LED test passed");
            }
        }

        // Set Green LED to 1 and Red/Blue LEDs to 0
        led.set_device(LedColor::Red, 0)?;
        led.set_device(LedColor::Green, 0)?;
        led.set_device(LedColor::Blue, 1)?;

        // Confirm if the user can see the Blue LED color (which should be off)
        if !question_prompt(
            Device::Led,
            MessageType::Confirm,
            "Can you see the Blue LED color?".to_string(),
        ) {
            log_message(Device::Led, MessageType::Fail, "LED test failed");
            return Ok(false); // Test failed
        } else {
            log_message(Device::Led, MessageType::Pass, "LED test passed");
        }

        Ok(true) // Test passed
    }
}
