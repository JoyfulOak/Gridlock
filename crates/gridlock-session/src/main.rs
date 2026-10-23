use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let config = match std::env::args_os().nth(1) {
        Some(config) => PathBuf::from(config),
        None => match default_config_path() {
            Ok(path) => path,
            Err(error) => {
                eprintln!("gridlock-session: {error}");
                return ExitCode::FAILURE;
            }
        },
    };
    if !config.is_file() {
        eprintln!(
            "gridlock-session: Sway config does not exist: {}",
            config.display()
        );
        eprintln!("Copy config/defaults/sway.conf to ~/.config/gridlock/sway.conf first.");
        return ExitCode::FAILURE;
    }
    match Command::new("sway").arg("-c").arg(&config).status() {
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(error) => {
            eprintln!("gridlock-session: could not start Sway: {error}");
            ExitCode::FAILURE
        }
    }
}

fn default_config_path() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join("gridlock/sway.conf"));
    }
    let home =
        std::env::var_os("HOME").ok_or("HOME is unset; pass a Sway config path explicitly")?;
    Ok(PathBuf::from(home).join(".config/gridlock/sway.conf"))
}
