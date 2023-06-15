use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeviceConfig {
    pub name: String,
    pub interfaces: Interfaces,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Interfaces {
    pub display: Display,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Display {
    pub device: String,
}
