use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};
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
            currnet_now: String::new(),
        };

        log_message(
            Device::Battery,
            MessageType::Action,
            "Setting Battery Device",
        );

        // will be replaced with actual behavior using SDK
        battery.set_device("/sys/class/power_supply/bq27441-0/uevent");

        log_message(
            Device::Battery,
            MessageType::Action,
            "Please plug-in the charger in 5 seconds to continue",
        );
        std::thread::sleep(std::time::Duration::from_secs(5));
        // We need to ask the user to plug in the charger.
        // If the charger is plugged in, we will check if the battery is charging.
        let user_response = question_prompt(
            Device::Battery,
            MessageType::Confirm,
            "is battery charging ?".to_string(),
        );
        if user_response {
            log_message(Device::Battery, MessageType::Action, "Battery Charging");
        } else {
            log_message(Device::Battery, MessageType::Action, "Battery Discharging");
        }

        log_message(
            Device::Battery,
            MessageType::Action,
            "Please unplug the charger in 5 seconds to continue",
        );
        //wait for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));
        // We need to ask the user to unplug the charger.
        // If the charger is unplugged, we will check if the battery is discharging.
        let user_response = question_prompt(
            Device::Battery,
            MessageType::Confirm,
            "is battery discharging ?".to_string(),
        );
        if user_response {
            log_message(Device::Battery, MessageType::Action, "Battery Charging");
        } else {
            log_message(Device::Battery, MessageType::Action, "Battery Discharging");
        }

        Ok(true)
    }
}
