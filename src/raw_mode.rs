use std::process::Command;

const ANSI_HIDE: &str = "\x1B[?25l";
const ANSI_SHOW: &str = "\x1B[?25h";

pub struct RawMode;

impl RawMode {
    pub fn enter() -> Self {
        let _ = Command::new("stty")
            .arg("-icanon")
            .arg("-echo")
            .spawn()
            .and_then(|mut child| child.wait());
        print!("{ANSI_HIDE}");
        Self
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        let _ = Command::new("stty")
            .arg("icanon")
            .arg("echo")
            .spawn()
            .and_then(|mut child| child.wait());
        print!("{ANSI_SHOW}");
    }
}