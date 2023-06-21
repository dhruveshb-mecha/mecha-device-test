use crate::test_base::{log_message, Device, MessageType, TestAssertion};
use mecha_mic::{arecord::MechaAudio, AudioInterface};

pub struct AudioRecord;

impl TestAssertion for AudioRecord {
    fn describe(&self) -> &str {
        "Audio Record test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        log_message(
            Device::Audio,
            MessageType::Test,
            "Recording Audio for 15 seconds",
        );
        MechaAudio::record_audio(15, "test.wav");
        //wait for 15 seconds
        log_message(
            Device::Audio,
            MessageType::Info,
            "Recording Audio for 15 seconds",
        );
        Ok(true)
    }
}
