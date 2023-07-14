use anyhow::{anyhow, Result};
use std::process::{Command, Output};

pub trait CameraInterface {
    fn capture_image(&self, image_name: &str) -> Result<()>;
    fn preview_image(&self, image_name: &str) -> Result<()>;
}

pub struct Camera;

impl CameraInterface for Camera {
    fn capture_image(&self, image_name: &str) -> Result<()> {
        // Build the command
        let command_output: Output = Command::new("gst-launch-1.0")
            .args(&[
                "-v",
                "v4l2src",
                &format!("device=/dev/video0"),
                "num-buffers=1",
                "!",
                "jpegenc",
                "!",
                "filesink",
                &format!("location={}", image_name),
            ])
            .output()
            .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

        if command_output.status.success() {
            println!("Image captured and saved to '{}'", image_name);
            Ok(())
        } else {
            println!("Failed to capture the image");
            Err(anyhow!("Failed to capture the image"))
        }
    }

    fn preview_image(&self, image_name: &str) -> Result<()> {
        let command_output = Command::new("weston-image")
            .args(&[image_name])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()
            .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

        if command_output.status.success() {
            Ok(())
        } else {
            Err(anyhow!("Failed to preview the image"))
        }
    }
}
