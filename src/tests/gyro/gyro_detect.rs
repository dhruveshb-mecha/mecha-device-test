use mecha_gyro::{Gyroscope, GyroscopeTrait};

use crate::test_base::{log_message, Device, MessageType, TestAssertion};

pub struct GyroDetect {
    //add paths for 3 axix
    pub x_axis_path: String,
    pub y_axis_path: String,
    pub z_axis_path: String,
}

impl TestAssertion for GyroDetect {
    fn describe(&self) -> &str {
        "Gyro Detection test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let mut gyro = Gyroscope {
            x_axis: String::new(),
            y_axis: String::new(),
            z_axis: String::new(),
        };

        log_message(Device::Motion, MessageType::Action, "Setting Gyro Device");

        gyro.set_device(&self.x_axis_path, &self.y_axis_path, &self.z_axis_path);

        //printx,y,z axis paths
        // println!("x_axis_path: {}", self.x_axis_path);
        // println!("y_axis_path: {}", self.y_axis_path);
        // println!("z_axis_path: {}", self.z_axis_path);

        //if gyro.set_device(&self.x_axis_path, &self.y_axis_path, &self.z_axis_path)  retuns empty string, then return false else true

        if gyro.get_device().0.is_empty()
            || gyro.get_device().1.is_empty()
            || gyro.get_device().2.is_empty()
        {
            log_message(Device::Motion, MessageType::Fail, "Gyro Device not set");
            return Ok(false);
        } else {
            log_message(Device::Motion, MessageType::Pass, "Gyro Device Set");
            return Ok(true);
        }
    }
}
