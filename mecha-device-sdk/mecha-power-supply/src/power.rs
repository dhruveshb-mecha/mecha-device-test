use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct PowerSupply {
    name: String,
    r#type: String,
    status: String,
    present: bool,
    voltage_now: u32,
    current_now: i32,
    capacity: u8,
    capacity_level: String,
    temp: i32,
    technology: String,
    charge_full: u32,
    charge_now: u32,
    charge_full_design: u32,
    manufacturer: String,
}

pub trait PowerSupplyInfo {
    fn info(&self) -> String;
    fn set_device(&mut self, device: &str);
    fn get_device(&self) -> &str;
    fn get_current(&self) -> Result<i64, std::io::Error>;
}

pub struct Battery {
    pub path: String,
    pub currnet_now: String,
}

impl PowerSupplyInfo for Battery {
    fn info(&self) -> String {
        "Battrery Info".to_string()
    }

    fn set_device(&mut self, device: &str) {
        self.path = device.to_owned();
    }

    fn get_device(&self) -> &str {
        &self.path
    }

    //to get current_now value read file from current_now path
    fn get_current(&self) -> Result<i64, std::io::Error> {
        let mut file = fs::File::open(&self.currnet_now)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        print!("file name {:?}", &file);
        print!("{}", contents);

        let value: i64 = contents
            .trim()
            .parse()
            .map_err(|e: ParseIntError| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(value)
    }
}
