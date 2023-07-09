use std::io::{Write};
use std::process::{Command};

pub fn set_tdp(tdp: u32) -> Result<u32, std::io::Error> {
    let target_tdp = tdp * 1000;
    let boost_tdp = target_tdp + 2000;

    let command = ["ryzenadj", &format!("--stapm-limit={}", target_tdp), &format!("--fast-limit={}", boost_tdp), &format!("--slow-limit={}", target_tdp)];
    Ok(run_command(&command).expect("Failed to run command").parse().unwrap())
}

fn run_command(command: &[&str]) -> Result<String, std::io::Error> {
    let mut full_command = vec!["sudo", "-S"];
    full_command.extend_from_slice(command);

    let mut child = Command::new(full_command[0])
        .args(&full_command[1..])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    // Pass the "gamer" string as stdin
    let stdin = child.stdin.as_mut().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to open stdin")
    })?;
    stdin.write_all(b"gamer\n")?;

    let output = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    Ok(stdout)
}
