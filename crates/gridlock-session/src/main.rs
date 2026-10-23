use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    match std::env::args_os().skip(1).next() {
        Some(config) => match Command::new("sway").arg("-c").arg(config).status() {
            Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
            Err(error) => {
                eprintln!("gridlock-session: could not start Sway: {error}");
                ExitCode::FAILURE
            }
        },
        None => {
            eprintln!("usage: gridlock-session <sway-config>");
            eprintln!("Gridlock session startup remains explicit; it does not modify login-manager or system configuration.");
            ExitCode::FAILURE
        }
    }
}
