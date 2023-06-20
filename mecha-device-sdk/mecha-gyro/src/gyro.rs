use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub trait GyroscopeTrait {
    fn set_device(&mut self, x_axis: &str, y_axis: &str, z_axis: &str);
    fn get_device(&self) -> (&str, &str, &str);
    fn info(&self) -> Result<(), io::Error>;
    fn get_data(&self) -> Result<(i32, i32, i32), io::Error>;
}

pub struct Gyroscope {
    pub x_axis: String,
    pub y_axis: String,
    pub z_axis: String,
}

impl GyroscopeTrait for Gyroscope {
    fn set_device(&mut self, x_axis: &str, y_axis: &str, z_axis: &str) {
        self.x_axis = x_axis.to_string();
        self.y_axis = y_axis.to_string();
        self.z_axis = z_axis.to_string();
    }

    fn get_device(&self) -> (&str, &str, &str) {
        (&self.x_axis, &self.y_axis, &self.z_axis)
    }

    fn info(&self) -> Result<(), io::Error> {
        let file_x = File::open(&self.x_axis)?;
        let reader_x = BufReader::new(file_x);
        let line_x = reader_x.lines().next().unwrap()?;
        println!("X: {}", line_x); // Modify as per your requirement

        let file_y = File::open(&self.y_axis)?;
        let reader_y = BufReader::new(file_y);
        let line_y = reader_y.lines().next().unwrap()?;
        println!("Y: {}", line_y); // Modify as per your requirement

        let file_z = File::open(&self.z_axis)?;
        let reader_z = BufReader::new(file_z);
        let line_z = reader_z.lines().next().unwrap()?;
        println!("Z: {}", line_z); // Modify as per your requirement

        Ok(())
    }

    fn get_data(&self) -> Result<(i32, i32, i32), io::Error> {
        let x_raw = read_gyroscope_value(&self.x_axis)?;
        let y_raw = read_gyroscope_value(&self.y_axis)?;
        let z_raw = read_gyroscope_value(&self.z_axis)?;
        Ok((x_raw, y_raw, z_raw))
    }
}

fn read_gyroscope_value(device_path: &str) -> Result<i32, io::Error> {
    let file = File::open(device_path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    let value = line.trim().parse::<i32>().unwrap();
    Ok(value)
}
