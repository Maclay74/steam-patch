use std::process::{Command, Output};

pub fn set_tdp(tdp: u32) -> std::io::Result<u32> {
    let target_tdp = tdp * 1000;
    let boost_tdp = target_tdp + 2000;

    let command = ["ryzenadj", &format!("--stapm-limit={}", target_tdp), &format!("--fast-limit={}", boost_tdp), &format!("--slow-limit={}", target_tdp)];
    run_command(&command).unwrap();
    Ok(tdp)
}

fn run_command(command: &[&str]) -> std::io::Result<Output> {
    let mut full_command = vec!["sudo", "-S"];
    full_command.extend_from_slice(command);

    Command::new(full_command[0])
        .args(&full_command[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .output()
}
