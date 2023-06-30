use std::{
    process::{exit, Command},
    thread,
    time::Duration,
};

use mecha_display::{Display, DisplayInterface};
use mecha_power_supply::{Battery, PowerSupplyInfo};

use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};

pub fn camera_preview() {
    let command = Command::new("gst-launch-1.0")
        .arg("v4l2src")
        .arg("device=/dev/video0")
        .arg("!")
        .arg("video/x-raw,width=1280,height=720,framerate=30/1")
        .arg("!")
        .arg("vpuenc_h264")
        .arg("!")
        .arg("h264parse")
        .arg("!")
        .arg("vpudec")
        .arg("!")
        .arg("imxvideoconvert_g2d")
        .arg("!")
        .arg("queue")
        .arg("!")
        .arg("waylandsink")
        .spawn();

    match command {
        Ok(mut child) => {
            // Wait for 15 seconds
            thread::sleep(Duration::from_secs(15));

            // Terminate the GStreamer command
            child.kill().expect("Failed to kill the GStreamer command.");
        }
        Err(e) => {
            eprintln!("Failed to execute GStreamer command: {}", e);
            exit(1);
        }
    }
}

fn play_audio(file_path: String) {
    let command = Command::new("aplay").arg(file_path).spawn();

    match command {
        Ok(mut child) => {
            // Wait for 15 seconds before terminating the audio playback.
            thread::sleep(Duration::from_secs(15));

            // Terminate the audio playback.
            child.kill().expect("Failed to kill the audio playback.");
        }
        Err(e) => {
            eprintln!("Failed to execute aplay command: {}", e);
            exit(1);
        }
    }
}

pub struct PowerTest4 {
    pub display_path: String,
    pub camera_path: String,
    pub current_now: String,
    pub audio_path: String,
}

impl TestAssertion for PowerTest4 {
    fn describe(&self) -> &str {
        "Power Test 4"
    }

    fn test(&self) -> anyhow::Result<bool> {
        log_message(Device::Power, MessageType::Test, "Power Test 3 Started");

        log_message(Device::Power, MessageType::Info, "Printing Test Conditions");

        log_message(
            Device::Power,
            MessageType::Action,
            "Setting Display Brightness to 80%",
        );

        let mut display = Display {
            path: String::new(),
        };

        // Set the display path.
        display.set_device(&self.display_path);

        // Check if the display path is empty after calling set_device.
        if display.path.is_empty() {
            log_message(
                Device::Power,
                MessageType::Error,
                &format!("Unable to find display"),
            );
            return Ok(false);
        }

        if let Err(err) = display.set_brightness(204) {
            log_message(
                Device::Power,
                MessageType::Error,
                &format!("Failed to set display brightness: {}", err),
            );
            return Ok(false);
        }

        log_message(Device::Power, MessageType::Info, "Camera - On");
        log_message(Device::Power, MessageType::Info, "Audio - Off");

        //ask user that battery is charging or not based on that log info message that battery is charging or not
        let user_response = question_prompt(
            Device::Power,
            MessageType::Confirm,
            "Is the battery charging?".to_owned(),
        );

        if user_response {
            log_message(Device::Power, MessageType::Info, "Battery - Charging");
        } else {
            log_message(Device::Power, MessageType::Info, "Battery - Not Charging");
        }

        // Spawn a separate thread to run the GStreamer command for camera preview.
        let camera_thread = thread::spawn(camera_preview);

        let audio_path = self.audio_path.clone();
        //spawn a separate thread to run the audio
        let audio_thread = thread::spawn(|| play_audio(audio_path));

        // Get the current measurements while the camera preview is running.
        let mut total_current = 0;
        for _ in 0..5 {
            let battery = Battery {
                path: String::new(),
                currnet_now: self.current_now.clone(),
            };

            let current_now = battery.get_current()?;
            total_current += current_now;
            // Log each individual current measurement.
            log_message(
                Device::Power,
                MessageType::Info,
                &format!("Current Now: {}", current_now / 1000),
            );

            // Wait for 3 seconds between each measurement.
            thread::sleep(Duration::from_secs(3));
        }

        // Calculate the average current.
        let average_current = total_current / 5;
        log_message(
            Device::Power,
            MessageType::Info,
            &format!("Average Current usage: {}", average_current / 1000),
        );

        // Terminate the GStreamer command by waiting for the camera thread to finish.
        camera_thread.join().expect("Failed to join camera thread.");
        //terminate the audio thread by waiting for the audio thread to finish
        audio_thread.join().expect("Failed to join audio thread.");
        Ok(true)
    }
}
