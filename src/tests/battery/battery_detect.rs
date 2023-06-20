use crate::test_base::{question_prompt, TestAssertion};
use anyhow::Result;
use mecha_power_supply::{Battery, PowerSupply, PowerSupplyInfo};

pub(crate) struct BatteryDetect {
    pub device: String,
}

impl TestAssertion for BatteryDetect {
    fn describe(&self) -> &str {
        "Battery Detect"
    }

    fn test(&self) -> Result<bool> {
        let mut battery = Battery {
            path: String::new(),
        };

        // will be replaced with actual behavior using SDK
        battery.set_device("/sys/class/power_supply/bq27441-0/uevent");

        let baattery_path = battery.get_device();

        // we need to check if battert path is retun empty string
        if baattery_path.is_empty() {
            println!("Battery not found");
            return Ok(false);
        }

        Ok(true)
    }
}
