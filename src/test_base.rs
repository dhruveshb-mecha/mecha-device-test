use std::{
    fs::OpenOptions,
    io::{self, Write},
    time::SystemTime,
};

use anyhow::Result;

pub trait TestAssertion {
    fn describe(&self) -> &str;
    fn test(&self) -> Result<bool>;
}

pub struct TestRunner {
    pub suit: Vec<Box<dyn TestAssertion>>,
    pub test_count: usize,
    pub test_passed: usize,
    pub test_failed: usize,
    pub test_runner: fn(),
}

impl TestRunner {
    pub fn run(&mut self) -> Result<()> {
        for test in &self.suit {
            let result = test.test()?;
            self.test_count += 1;
            if result {
                self.test_passed += 1;
            } else {
                self.test_failed += 1;
            }
        }
        Ok(())
    }
}

pub fn question_prompt(device: Device, message_type: MessageType, message: String) -> bool {
    print!("[{:?}] {:?}: {} [y/n] ", device, message_type, message);

    let mut answer = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim();

    match answer {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => {
            println!("Invalid input, please try again");
            question_prompt(device, message_type, message)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Device {
    Display,
    Motion,
    Battery,
    Led,
    Audio,
    Camera, // Add more devices as needed
}
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Test,
    Info,
    Action,
    Confirm,
    Pass,
    Fail,
    Error,
    // Add more message types as needed
}

pub fn log_message(device: Device, message_type: MessageType, message: &str) {
    println!("[{:?}] {:?}: {}", device, message_type, message);
    write_to_file(format!("[{:?}] {:?}: {}", device, message_type, message));
}

fn write_to_file(log_message: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file.");

    if let Err(e) = writeln!(file, "{}", log_message) {
        eprintln!("Error writing to log file: {}", e);
    }
}
