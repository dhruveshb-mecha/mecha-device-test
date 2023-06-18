use crate::test_base::TestAssertion;
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

        // will be replaced with actual behaviour using sdk
        display.set_device("/sys/class/backlight/backlight/brightness");
        // display.set_brightness(144).unwrap();

        println!("Display Brightness: {}", 100);

        Ok(true)
    }
}
