extern crate sysfs_pwm;
use std::env;
use sysfs_pwm::{Pwm, Result};

pub fn pwm_device(period_ns: u32, duty_cycle_ns: u32) -> Result<()> {
    let pwm = Pwm::new(0, 0)?;
    pwm.with_exported(|| {
        pwm.enable(true)?;
        pwm.set_period_ns(period_ns)?;
        pwm.set_duty_cycle_ns(duty_cycle_ns)?;
        std::thread::sleep(std::time::Duration::from_secs(2));
        pwm.enable(false)?;
        Ok(())
    })?;

    Ok(())
}
