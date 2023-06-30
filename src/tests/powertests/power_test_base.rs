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
