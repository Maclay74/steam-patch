use std::io;
use std::process::{Command, Output};

pub fn run_command(command: &[&str]) -> io::Result<Output> {
    let child = Command::new(command[0])
        .args(&command[1..])
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    Ok(output)
}