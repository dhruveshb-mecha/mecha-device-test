use std::fs::File;
use std::io::{BufReader, Read, Result, Write};

pub trait AdcDevice {
    fn set_device(&mut self, channel_1_path: String, channel_2_path: String) -> Result<()>;
    fn get_device(&self) -> Option<(&str, &str)>;
    fn read_channel_1(&mut self) -> Result<i16>;
    fn read_channel_2(&mut self) -> Result<i16>;
    fn set_sampling_frequency(&mut self, sampling_frequency: String) -> Result<()>;
    fn get_sampling_frequency(&self) -> Option<String>;
}

pub struct Adc {
    pub channel_1_path: String,
    pub channel_2_path: String,
    pub sampling_frequency: String,
}

impl AdcDevice for Adc {
    fn set_device(&mut self, channel_1_path: String, channel_2_path: String) -> Result<()> {
        self.channel_1_path = channel_1_path;
        self.channel_2_path = channel_2_path;
        Ok(())
    }

    fn get_device(&self) -> Option<(&str, &str)> {
        Some((&self.channel_1_path, &self.channel_2_path))
    }

    fn read_channel_1(&mut self) -> Result<i16> {
        let mut buffer = String::new();
        File::open(&self.channel_1_path)?.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<i16>().unwrap())
    }

    fn read_channel_2(&mut self) -> Result<i16> {
        let mut buffer = String::new();
        File::open(&self.channel_2_path)?.read_to_string(&mut buffer)?;
        Ok(buffer.trim().parse::<i16>().unwrap())
    }

    fn set_sampling_frequency(&mut self, sampling_frequency: String) -> Result<()> {
        let mut file = File::create(&self.sampling_frequency)?;
        file.write_all(sampling_frequency.as_bytes())?;
        Ok(())
    }

    fn get_sampling_frequency(&self) -> Option<String> {
        let mut buf_reader = BufReader::new(File::open(&self.sampling_frequency).ok()?);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).ok()?;
        Some(contents)
    }
}
