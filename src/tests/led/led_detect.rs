use mecha_led::{Led, LedColor, LedInterface};

use crate::test_base::TestAssertion;

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
            red: String::new(),
            green: String::new(),
            blue: String::new(),
        };

        let _red_led = led.set_device(LedColor::Red, 1);
        let _green_led = led.set_device(LedColor::Green, 1);
        let _blue_led = led.set_device(LedColor::Blue, 1);

        // if the values of red, green and blue are 1, then return true else false

        if led.get_device(LedColor::Red).unwrap() == 1
            && led.get_device(LedColor::Green).unwrap() == 1
            && led.get_device(LedColor::Blue).unwrap() == 1
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}
