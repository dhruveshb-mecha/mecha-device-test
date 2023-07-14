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
            Ok(())
        } else {
            Err(anyhow!("Failed to capture the image"))
        }
    }

    fn preview_image(&self, image_name: &str) -> Result<()> {
        let mut output = Command::new("timeout")
            .arg("5")
            .arg("weston-image")
            .arg(image_name)
            .spawn()
            .map_err(|e| anyhow!("Failed to execute command: {}", e))?;

        match output.wait() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(anyhow!("Failed to preview the image"))
                }
            }
            Err(err) => Err(anyhow!("Failed to wait for command completion: {}", err)),
        }
    }
}
