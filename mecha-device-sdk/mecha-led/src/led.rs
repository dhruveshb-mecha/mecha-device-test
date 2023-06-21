use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub enum LedColor {
    Red,
    Green,
    Blue,
}

pub trait LedInterface {
    fn set_device(&mut self, color: LedColor, value: u8) -> Result<(), io::Error>;
    fn get_device(&self, color: LedColor) -> Result<i32, io::Error>;
    fn info(&self) -> Result<(i32, i32, i32), io::Error>;
    // fn get_data(&self, color: LedColor) -> Result<i32, io::Error>;
}

pub struct Led {
    pub red: String,
    pub green: String,
    pub blue: String,
}

impl LedInterface for Led {
    fn set_device(&mut self, color: LedColor, value: u8) -> Result<(), io::Error> {
        let value_str = match value {
            0 => "0",
            1 => "1",
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid value")),
        };

        let path = match color {
            LedColor::Red => &self.red,
            LedColor::Green => &self.green,
            LedColor::Blue => &self.blue,
        };

        fs::write(path, value_str)?;
        Ok(())
    }

    fn get_device(&self, color: LedColor) -> Result<i32, io::Error> {
        let path = match color {
            LedColor::Red => &self.red,
            LedColor::Green => &self.green,
            LedColor::Blue => &self.blue,
        };

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let line = reader.lines().next();

        match line {
            Some(Ok(value)) => {
                if let Ok(int_value) = value.parse::<i32>() {
                    Ok(int_value)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid LED value",
                    ))
                }
            }
            Some(Err(err)) => Err(err),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "LED value not found",
            )),
        }
    }

    fn info(&self) -> Result<(i32, i32, i32), io::Error> {
        let red_value = self.get_device(LedColor::Red)?;
        let green_value = self.get_device(LedColor::Green)?;
        let blue_value = self.get_device(LedColor::Blue)?;

        Ok((red_value, green_value, blue_value))
    }
}
