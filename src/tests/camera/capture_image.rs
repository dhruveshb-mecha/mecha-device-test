use crate::test_base::{question_prompt, Device, MessageType, TestAssertion};
use anyhow::Result;
use mecha_camera::{Camera, CameraInterface};

pub struct CameraImageCapture;

impl TestAssertion for CameraImageCapture {
    fn describe(&self) -> &str {
        "Camera Image Capture"
    }

    fn test(&self) -> Result<bool> {
        let camera = Camera;
        camera.capture_image("test_image")?;
        print!("Capturing Image Please wait...");
        std::thread::sleep(std::time::Duration::from_secs(2));
        let user_response = question_prompt(
            Device::Camera,
            MessageType::Confirm,
            "Is the camera image captured?".to_string(),
        );
        if user_response {
            println!("Camera Image Captured");
        } else {
            println!("Camera Image Not Captured");
            return Ok(false);
        }

        Ok(true)
    }
}
