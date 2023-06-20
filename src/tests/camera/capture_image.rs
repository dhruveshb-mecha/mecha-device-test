use crate::test_base::{question_prompt, TestAssertion};
use anyhow::Result;
use mecha_camera::{Camera, CameraInterface};

pub struct CameraImageCapture;

impl TestAssertion for CameraImageCapture {
    fn describe(&self) -> &str {
        "Camera Image Capture"
    }

    fn test(&self) -> Result<bool> {
        let mut camera = Camera;
        camera.capture_image("test_image")?;

        let user_response = question_prompt("Is the camera image captured?".to_owned());
        if user_response {
            println!("Camera Image Captured");
        } else {
            println!("Camera Image Not Captured");
            return Ok(false);
        }

        Ok(true)
    }
}
