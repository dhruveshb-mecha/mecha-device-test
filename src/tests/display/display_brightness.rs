use crate::test_base::{question_prompt, TestAssertion};
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

        // will be replaced with actual behavior using SDK
        display.set_device("/sys/class/backlight/backlight/brightness");

        // Brightness level to test (100% and 10%)
        let brightness_levels = [255, 16];

        for brightness in &brightness_levels {
            if let Err(err) = display.set_brightness(*brightness) {
                println!("Failed to set display brightness: {}", err);
                return Ok(false);
            }

            let user_response =
                question_prompt(format!("Is the display brightness set to {}%?", brightness));
            if user_response {
                println!("Display Brightness: {}", brightness);
            } else {
                let current_brightness = display.get_brightness()?;
                println!("Display Brightness: {}", current_brightness);
                return Ok(false);
            }
        }

        Ok(true)
    }
}
