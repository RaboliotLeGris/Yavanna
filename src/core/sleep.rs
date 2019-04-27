const COMMAND: &str = "shutdown";

pub fn at(hours: i32, minutes: i32) {
    let formated_arg = std::format!("{}:{}", hours, minutes);
    std::process::Command::new(COMMAND)
        .args(&["--no-wall", &formated_arg])
        .output()
        .expect("Fail to run at command");;
}

pub fn after(minutes: i32) {
    let formated_arg = std::format!("+{}", minutes);
    std::process::Command::new(COMMAND)
        .args(&["--no-wall", &formated_arg])
        .output()
        .expect("Fail to run After command");
}

pub fn cancel() {
    std::process::Command::new(COMMAND)
        .args(&["-c"])
        .output()
        .expect("Fail to run Cancer command");;
}