const COMMAND: &str = "shutdown";

pub fn at(hours: String, minutes: String) {
    let formated_arg = std::format!("{}:{}", hours, minutes);
    std::process::Command::new(COMMAND)
        .args(&["--no-wall", &formated_arg])
        .output();
}

pub fn after(minutes: String) {
    let formated_arg = std::format!("+{}", minutes);
    std::process::Command::new(COMMAND)
        .args(&["--no-wall", &formated_arg])
        .output();
}

pub fn cancel() {
    std::process::Command::new(COMMAND)
        .args(&["-c"])
        .output();
}