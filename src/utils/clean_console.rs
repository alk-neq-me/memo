use std::process::Command;

pub fn clean_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("clear")
            .status()
            .unwrap();
    }
}
