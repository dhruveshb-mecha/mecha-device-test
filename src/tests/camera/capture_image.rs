use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};
use anyhow::Result;
use mecha_camera::{Camera, CameraInterface};

pub struct CameraImageCapture;

impl TestAssertion for CameraImageCapture {
    fn describe(&self) -> &str {
        "Camera Image Capture"
    }

    fn test(&self) -> Result<bool> {
        let camera = Camera;

        // Ask the user if they have pointed the camera at the proper location
        let user_response = question_prompt(
            Device::Camera,
            MessageType::Confirm,
            "Have you pointed the camera at the proper location for image capture?".to_string(),
        );

        if user_response {
            log_message(Device::Camera, MessageType::Action, "Capturing Image");
            let image_name = "image.jpg";
            camera.capture_image(image_name)?;
            log_message(Device::Camera, MessageType::Action, "Image Captured");

            log_message(Device::Camera, MessageType::Action, "Previewing Image");
            camera.preview_image(image_name)?;

            // Ask the user if the image was previewed correctly
            let user_response = question_prompt(
                Device::Battery,
                MessageType::Confirm,
                "Are you able to see the Image?".to_string(),
            );

            if user_response {
                log_message(Device::Camera, MessageType::Action, "Image Previewed");
            } else {
                log_message(Device::Camera, MessageType::Action, "Image Preview Failed");
            }
        } else {
            log_message(
                Device::Camera,
                MessageType::Action,
                "Camera not pointed at the proper location",
            );
        }

        Ok(true)
    }
}
