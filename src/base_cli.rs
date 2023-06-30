use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeviceConfig {
    pub name: String,
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Interfaces {
    pub display: Display,
    pub battery: Battery,
    pub gyroscope: Gyroscope,
    pub led: Led,
    pub adc: Adc,
    pub camera: Camera,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Display {
    pub device: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Battery {
    pub device: String,
    pub capacity: String,
    pub voltage: String,
    pub current: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Gyroscope {
    pub x_axis: String,
    pub y_axis: String,
    pub z_axis: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Led {
    pub red_led: String,
    pub green_led: String,
    pub blue_led: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Adc {
    pub channel_1: String,
    pub channel_2: String,
    pub sampling_frequency: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Camera {
    pub device: String,
}
