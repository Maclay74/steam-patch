use std::process::{Command, Output};
use std::io::{self, Write};

pub fn set_tdp(tdp: i32) -> io::Result<i32> {
    let target_tdp = tdp * 1000;
    let boost_tdp = target_tdp + 2000;

    let command = ["ryzenadj", &format!("--stapm-limit={}", target_tdp), &format!("--fast-limit={}", boost_tdp), &format!("--slow-limit={}", target_tdp)];
    match run_command(&command) {
        Ok(_) => Ok(tdp),
        Err(err) => Err(err),
    }
}

fn run_command(command: &[&str]) -> io::Result<Output> {
    let mut full_command = vec!["sudo", "-S"];
    full_command.extend_from_slice(command);

    let mut child = Command::new(full_command[0])
        .args(&full_command[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all("gamer\n".as_bytes())?;
    }

    let output = child.wait_with_output()?;
    Ok(output)
}