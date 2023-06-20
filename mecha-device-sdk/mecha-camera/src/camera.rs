use anyhow::Error;
use gstreamer::{prelude::*, ClockTime};
use log::{error, info};

pub trait CameraInterface {
    fn capture_image(&self, image_name: &str) -> Result<(), Error>;
    fn capture_video(&self, path: &str) -> Result<(), Error>;
}

pub struct Camera;

impl CameraInterface for Camera {
    fn capture_image(&self, image_name: &str) -> Result<(), Error> {
        gstreamer::init()?;

        // Create the elements
        let source = gstreamer::ElementFactory::make("v4l2src").build()?;
        let image_name = format!("{}.jpg", image_name);

        let converter = gstreamer::ElementFactory::make("jpegenc").build()?;
        // let encoder = gstreamer::ElementFactory::make("vpuenc_h264").build()?;
        let filesink = gstreamer::ElementFactory::make("filesink")
            .property("location", image_name)
            .build()?;
        // Create the empty pipeline
        let pipeline = gstreamer::Pipeline::new(Some("image-capture-pipeline"));

        // Add elements to the pipeline
        pipeline.add_many(&[&source, &converter, &filesink])?;

        // Link the elements
        gstreamer::Element::link_many(&[&source, &converter, &filesink])?;

        // Start recording
        info!("Starting recording");
        pipeline.set_state(gstreamer::State::Playing)?;

        // Wait for the pipeline to be ready
        let bus = pipeline.bus().expect("Pipeline has no bus");
        let msg = bus.timed_pop_filtered(
            gstreamer::ClockTime::SECOND * 5,
            &[gstreamer::MessageType::Eos, gstreamer::MessageType::Error],
        );
        match msg {
            Some(msg) => match msg.view() {
                gstreamer::MessageView::Error(err) => {
                    error!(
                        "Error from {:?}: {} ({:?})",
                        err.src().map(|s| s.path_string()),
                        err.error(),
                        err.debug()
                    );
                }
                gstreamer::MessageView::Eos(..) => {
                    // i need to brake the loop here after 5 seconds
                    print!("Capturing Image Please wait...");
                    info!("Recording finished");
                }
                _ => unreachable!(),
            },
            None => {
                error!("Failed to receive message from the bus");
            }
        }

        // Stop the pipeline after 3 seconds
        let timeout = ClockTime::from_seconds(3);
        let clock = pipeline.clock().unwrap();
        clock.set_timeout(timeout);
        // Stop the pipeline
        info!("Stopping recording");
        pipeline.set_state(gstreamer::State::Null)?;

        Ok(())
    }

    fn capture_video(&self, path: &str) -> Result<(), Error> {
        println!("Capture Video: {}", path);
        Ok(())
    }
}
