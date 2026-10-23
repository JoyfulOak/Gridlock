use std::path::PathBuf;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let compositor = "hyprland";
    let mut config_arg = None;

    while let Some(argument) = args.next() {
        if argument == "--compositor" {
            let Some(value) = args.next() else {
                eprintln!("gridlock-session: --compositor requires hyprland");
                return ExitCode::FAILURE;
            };
            if value != "hyprland" {
                eprintln!("gridlock-session: Hyprland is the supported compositor");
                return ExitCode::FAILURE;
            }
        } else if config_arg.is_none() {
            config_arg = Some(PathBuf::from(argument));
        } else {
            eprintln!("usage: gridlock-session [--compositor hyprland] [config-path]");
            return ExitCode::FAILURE;
        }
    }

    let config = match config_arg {
        Some(config) => config,
        None => match default_config_path(compositor) {
            Ok(path) => path,
            Err(error) => {
                eprintln!("gridlock-session: {error}");
                return ExitCode::FAILURE;
            }
        },
    };
    if !config.is_file() {
        eprintln!(
            "gridlock-session: {compositor} config does not exist: {}",
            config.display()
        );
        eprintln!(
            "Copy the matching config/defaults/{compositor}.conf into ~/.config/gridlock/ first."
        );
        return ExitCode::FAILURE;
    }

    let program = "Hyprland";
    match Command::new(program).arg("-c").arg(&config).status() {
        Ok(status) => ExitCode::from(status.code().unwrap_or(1) as u8),
        Err(error) => {
            eprintln!("gridlock-session: could not start {program}: {error}");
            ExitCode::FAILURE
        }
    }
}

fn default_config_path(compositor: &str) -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join(format!("gridlock/{compositor}.conf")));
    }
    let home = std::env::var_os("HOME")
        .ok_or("HOME is unset; pass a compositor config path explicitly")?;
    Ok(PathBuf::from(home).join(format!(".config/gridlock/{compositor}.conf")))
}
