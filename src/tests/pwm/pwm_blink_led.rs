use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};

pub struct PWM;

impl TestAssertion for PWM {
    fn describe(&self) -> &str {
        "PWM"
    }

    fn test(&self) -> anyhow::Result<bool> {
        log_message(Device::Pwm, MessageType::Test, "Pwm Test Started");

        question_prompt(
            Device::Pwm,
            MessageType::Confirm,
            "Have you added pwm to device ?".to_string(),
        );

        mecha_pwm::pwm_device(1000000, 500000)?;

        // if user answer yes, then test passed otherwise failed
        let result = question_prompt(
            Device::Pwm,
            MessageType::Test,
            "Did you see the LED blinking?".to_string(),
        );

        if result {
            log_message(Device::Pwm, MessageType::Test, "Pwm Test Passed");
            Ok(true)
        } else {
            log_message(Device::Pwm, MessageType::Test, "Pwm Test Failed");
            Ok(false)
        }
    }
}
