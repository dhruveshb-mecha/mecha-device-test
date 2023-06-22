use crate::test_base::{log_message, question_prompt, Device, MessageType, TestAssertion};
use mecha_adc::{Adc, AdcDevice};

pub struct AdcTest {
    pub channel_1_path: String,
    pub channel_2_path: String,
    pub sampling_frequency: String,
}

impl TestAssertion for AdcTest {
    fn describe(&self) -> &str {
        "ADC Test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let mut adc = Adc {
            channel_1_path: self.channel_1_path.clone(),
            channel_2_path: self.channel_2_path.clone(),
            sampling_frequency: self.sampling_frequency.clone(),
        };

        log_message(Device::Adc, MessageType::Test, "Adc Test Started");

        log_message(
            Device::Adc,
            MessageType::Info,
            "Please attach the ADC device.",
        );

        let attached = question_prompt(
            Device::Adc,
            MessageType::Confirm,
            "Is the ADC device attached?".to_string(),
        );

        //log message that attaches peripheral in 5 seconds
        log_message(
            Device::Adc,
            MessageType::Info,
            "Please attach the ADC device in 5 seconds.",
        );

        //sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));

        if !attached {
            log_message(Device::Adc, MessageType::Test, "Adc Test Failed");
            return Ok(false);
        } else {
            log_message(Device::Adc, MessageType::Test, "Adc Test Started");

            //log values of channel 1 and channel 2
            log_message(
                Device::Adc,
                MessageType::Info,
                &format!(
                    "Channel 1: {} Channel 2: {}",
                    adc.read_channel_1()?,
                    adc.read_channel_2()?
                ),
            );

            //log message to user to change values of the ADC device
            log_message(
                Device::Adc,
                MessageType::Info,
                "Please change the values of the ADC device.",
            );

            // wait for 5 seconds
            std::thread::sleep(std::time::Duration::from_secs(5));

            //log values of channel 1 and channel 2
            log_message(
                Device::Adc,
                MessageType::Info,
                &format!(
                    "Channel 1: {} Channel 2: {}",
                    adc.read_channel_1()?,
                    adc.read_channel_2()?
                ),
            );

            // ask the user to check ADC device values
            let result = question_prompt(
                Device::Adc,
                MessageType::Test,
                "Did you see the values of the ADC device changed?".to_string(),
            );

            if !result {
                log_message(Device::Adc, MessageType::Test, "Adc Test Failed");
                return Ok(false);
            }

            return Ok(true);
        }
    }
}
