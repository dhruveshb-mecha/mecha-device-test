use mecha_cpu::{CpuInfo, CPU};

use crate::test_base::{log_message, Device, MessageType, TestAssertion};

pub struct CpuStressTest;

impl TestAssertion for CpuStressTest {
    fn describe(&self) -> &str {
        "CPU Stress Test"
    }

    fn test(&self) -> anyhow::Result<bool> {
        let cpu = CPU;

        log_message(
            Device::Cpu,
            MessageType::Action,
            "Detecting CPU temperature",
        );
        let zone = 0; // Specify the thermal zone (0 or 1) for the CPU temperature you want to read
        match cpu.cpu_temp(zone) {
            Ok(temp) => {
                let message = format!("CPU temperature (zone {}): {:.2}°C", zone, temp);
                log_message(Device::Cpu, MessageType::Confirm, &message);
            }
            Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
        }

        let cpu_max_prime = 90000;
        log_message(Device::Cpu, MessageType::Action, "Running CPU stress test");
        match cpu.stress_test_cpu(cpu_max_prime) {
            Ok(()) => {
                log_message(Device::Cpu, MessageType::Pass, "CPU stress test passed");
            }
            Err(e) => {
                let error_message = format!("CPU stress test failed: {}", e);
                log_message(Device::Cpu, MessageType::Error, &error_message);
                eprintln!("Failed to execute command: {}", e)
            }
        }

        match cpu.cpu_temp(zone) {
            Ok(temp) => {
                let message = format!(
                    "CPU temperature after stress test (zone {}): {:.2}°C",
                    zone, temp
                );
                log_message(Device::Cpu, MessageType::Confirm, &message);
            }
            Err(e) => eprintln!("Failed to read CPU temperature: {}", e),
        }

        Ok(true)
    }
}
