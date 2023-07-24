use std::env;
use std::io;
use std::process::{Command, Output};
use sysinfo::{ProcessExt, SystemExt};

#[allow(dead_code)]
pub fn run_command(command: &[&str]) -> io::Result<Output> {
    let child = Command::new(command[0])
        .args(&command[1..])
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    Ok(output)
}

#[allow(dead_code)]
pub fn get_username() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return String::from("gamer");
    }

    let arg = &args[1];

    if arg.starts_with("--user=") {
        let username = arg.trim_start_matches("--user=");
        String::from(username)
    } else {
        String::from("gamer")
    }
}

#[allow(dead_code)]
fn is_steam_running() -> bool {
    let mut sys = sysinfo::System::new_all();

    // We need to update the system value to get the fresh process list
    sys.refresh_all();

    sys.processes()
        .values()
        .any(|process| process.name() == "steam")
}
