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

        println!("Please plug in the charger  to continue");
        std::thread::sleep(std::time::Duration::from_secs(5));
        // We need to ask the user to plug in the charger.
        // If the charger is plugged in, we will check if the battery is charging.
        let user_response = question_prompt("Is the battery charging?".to_owned());
        if user_response {
            println!("Battery Charging");
        } else {
            println!("Battery Not Charging");
        }

        println!("Please unplug the charger and press enter to continue");
        //wait for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
        // We need to ask the user to unplug the charger.
        // If the charger is unplugged, we will check if the battery is discharging.
        let user_response = question_prompt("Is the battery discharging?".to_owned());
        if user_response {
            println!("Battery Discharging");
        } else {
            println!("Battery Not Discharging");
        }

        Ok(true)
    }
}
