use crate::test_base::{question_prompt, TestAssertion};
use anyhow::Result;
use mecha_power_supply::{Battery, PowerSupply, PowerSupplyInfo};

pub(crate) struct BatteryCharging {
    pub device: String,
}

impl TestAssertion for BatteryCharging {
    fn describe(&self) -> &str {
        "Battery Detect"
    }

    fn test(&self) -> Result<bool> {
        let mut battery = Battery {
            path: String::new(),
        };

        // will be replaced with actual behavior using SDK
        battery.set_device("/sys/class/power_supply/bq27441-0/uevent");

        println!("Please plugin the charger and press enter to continue");
        // we need ro ask user ro plugin then if he plugin the charger we need to check if the battery is charging or not and then for discarging
        let user_response = question_prompt("Is the battery charging?".to_owned());
        if user_response {
            println!("Battery Charging");
        } else {
            println!("Battery Not Charging");
            return Ok(false);
        }

        println!("Please unplug the charger and press enter to continue");
        let user_response = question_prompt("Is the battery discharging?".to_owned());
        if user_response {
            println!("Battery Discharging");
        } else {
            println!("Battery Not Discharging");
            return Ok(false);
        }

        Ok(true)
    }
}
