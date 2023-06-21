use mecha_gyro::{Gyroscope, GyroscopeTrait};

use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};

pub struct GyroData {
    //add paths for 3 axix
    pub x_axis_path: String,
    pub y_axis_path: String,
    pub z_axis_path: String,
}

impl TestAssertion for GyroData {
    fn describe(&self) -> &str {
        "Gyro Detection test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let mut gyro = Gyroscope {
            x_axis: String::new(),
            y_axis: String::new(),
            z_axis: String::new(),
        };
        log_message(Device::Motion, MessageType::Info, "Getting Gyro Data");

        gyro.set_device(&self.x_axis_path, &self.y_axis_path, &self.z_axis_path);

        print!("x axis path : {}", gyro.x_axis);
        print!("y axis path : {}", gyro.y_axis);
        print!("z axis path : {}", gyro.z_axis);

        let (x_raw, y_raw, z_raw) = gyro.get_data()?;

        //printx,y,z values in log message
        log_message(
            Device::Motion,
            MessageType::Info,
            &format!("x : {}, y : {}, z : {} ", x_raw, y_raw, z_raw),
        );

        log_message(
            Device::Motion,
            MessageType::Action,
            "Move the device in all directions and check if the values are changing",
        );

        //sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));

        let (x_raw, y_raw, z_raw) = gyro.get_data()?;

        //printx,y,z values in log message
        log_message(
            Device::Motion,
            MessageType::Info,
            &format!("x : {}, y : {}, z : {} ", x_raw, y_raw, z_raw),
        );

        let user_response = question_prompt(
            Device::Motion,
            MessageType::Confirm,
            "Are the values different from the previous values?".to_string(),
        );

        //if user_response is true then log message as pass
        if user_response {
            log_message(Device::Motion, MessageType::Pass, "Gyro Data is changing");
            Ok(true)
        } else {
            log_message(
                Device::Motion,
                MessageType::Fail,
                "Gyro Data is not changing",
            );
            Ok(false)
        }

        //printx,y,z axis paths
    }
}
