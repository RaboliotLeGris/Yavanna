use crate::core::error::YavannaError;
use std::error::Error;

const COMMAND: &str = "shutdown";

pub fn at(hours: u32, minutes: u32) -> Result<(), Box<dyn Error>> {
    let formatted_arg = std::format!("{}:{}", hours, minutes);
    std::process::Command::new(COMMAND)
        .args(&["--no-wall", &formatted_arg])
        .output()?;
    Ok(())
}

pub fn after(minutes: u32) -> Result<(), Box<dyn Error>> {
    if minutes == 0 {
        return Err(Box::new(YavannaError::InvalidHour))
    }
    let formatted_arg = std::format!("+{}", minutes);
    std::process::Command::new(COMMAND)
        .args(&["--no-wall", &formatted_arg])
        .output()?;
    Ok(())
}

pub fn cancel() -> Result<(), Box<dyn Error>> {
    std::process::Command::new(COMMAND)
        .args(&["-c"])
        .output()?;
    Ok(())
}

// Find scheduled time -> /etc/crontab