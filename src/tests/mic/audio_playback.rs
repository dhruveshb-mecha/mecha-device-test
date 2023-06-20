use crate::test_base::{log_message, Device, MessageType, TestAssertion};
use mecha_mic::{arecord::MechaAudio, AudioInterface};

struct AudioPlayBack;

impl TestAssertion for AudioPlayBack {
    fn describe(&self) -> &str {
        "Audio Play test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        log_message(Device::Audio, MessageType::Test, "Play Audio file");
        log_message(Device::Audio, MessageType::Info, "Playing Audio");
        MechaAudio::play_audio("test.wav");

        //wait for 15 seconds

        std::thread::sleep(std::time::Duration::from_secs(15));

        let user_response = crate::test_base::question_prompt(
            Device::Audio,
            MessageType::Confirm,
            "Did you hear the audio?".to_string(),
        );

        //if user_response is true then log message as pass
        if user_response {
            log_message(Device::Audio, MessageType::Pass, "Audio is playing");
            Ok(true)
        } else {
            log_message(Device::Audio, MessageType::Fail, "Audio is not playing");
            Ok(false)
        }
    }
}
