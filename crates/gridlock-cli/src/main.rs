use gridlock_core::{Config, Direction};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if let Err(error) = run(std::env::args().skip(1).collect()) {
        eprintln!("gridlock: {error}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [command] if command == "status" => status(),
        [command, subcommand] if command == "config" && subcommand == "validate" => validate_config(),
        [command, subcommand, direction] if command == "window" && subcommand == "focus" => {
            focus(direction, None)
        }
        [command, subcommand, direction] if command == "window" && subcommand == "move" => {
            let direction: Direction = direction.parse()?;
            hyprland()?.move_focused(direction).map_err(|error| error.to_string())
        }
        [command, subcommand, direction, flag]
            if command == "window" && subcommand == "focus" && flag == "--no-center-pointer" =>
        {
            focus(direction, Some(false))
        }
        [command, subcommand, direction, flag]
            if command == "window" && subcommand == "focus" && flag == "--center-pointer" =>
        {
            focus(direction, Some(true))
        }
        [command, subcommand, number] if command == "workspace" && subcommand == "switch" => {
            hyprland()?.switch_workspace(parse_workspace(number)?).map_err(|error| error.to_string())
        }
        [command, subcommand, number] if command == "workspace" && subcommand == "move" => {
            hyprland()?.move_focused_to_workspace(parse_workspace(number)?).map_err(|error| error.to_string())
        }
        [command, subcommand] if command == "window" && subcommand == "close" => hyprland()?.close_focused().map_err(|error| error.to_string()),
        [command, subcommand] if command == "window" && subcommand == "fullscreen" => hyprland()?.toggle_fullscreen().map_err(|error| error.to_string()),
        [command, subcommand] if command == "window" && subcommand == "floating" => hyprland()?.toggle_floating().map_err(|error| error.to_string()),
        [command] if command == "terminal" => launch_terminal(),
        [command, operation] if command == "plugin" => plugin_command(operation, None),
        [command, operation, argument] if command == "plugin" => plugin_command(operation, Some(argument)),
        _ => Err("usage: gridlock status | config validate | window focus|move <direction> | window close|fullscreen|floating | workspace switch|move <1-9> | plugin list|inspect|install|enable|disable|remove".into()),
    }
}

fn hyprland() -> Result<gridlock_compositor_hyprland::HyprlandClient, String> {
    gridlock_compositor_hyprland::HyprlandClient::from_env().map_err(|error| error.to_string())
}

fn parse_workspace(value: &str) -> Result<u8, String> {
    let number: u8 = value
        .parse()
        .map_err(|_| "workspace must be a number from 1 through 9".to_string())?;
    if !(1..=9).contains(&number) {
        return Err("workspace must be a number from 1 through 9".into());
    }
    Ok(number)
}

fn launch_terminal() -> Result<(), String> {
    let config = read_config()?;
    Command::new(&config.terminal)
        .spawn()
        .map(|_| ())
        .map_err(|error| {
            format!(
                "could not launch configured terminal {:?}: {error}",
                config.terminal
            )
        })
}

fn plugin_command(operation: &str, argument: Option<&String>) -> Result<(), String> {
    if !matches!(
        operation,
        "list" | "inspect" | "install" | "enable" | "disable" | "remove"
    ) {
        return Err(format!("unknown plugin operation {operation:?}"));
    }
    let host =
        std::env::var_os("GRIDLOCK_PLUGIN_HOST").unwrap_or_else(|| "gridlock-plugin-host".into());
    let mut command = Command::new(host);
    command.arg(operation);
    if let Some(argument) = argument {
        command.arg(argument);
    }
    let status = command
        .status()
        .map_err(|error| format!("could not start plugin manager: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("plugin manager exited with {status}"))
    }
}

fn focus(direction: &str, center_override: Option<bool>) -> Result<(), String> {
    let direction: Direction = direction.parse()?;
    let config = read_config()?;
    let client = hyprland()?;
    let center = center_override.unwrap_or(config.pointer_follow_focus);
    let moved = client
        .focus_direction(
            direction,
            config.exclude_floating_from_directional_focus,
            center,
        )
        .map_err(|error| error.to_string())?;
    if !moved {
        return Err(format!(
            "no eligible window to the {direction:?}; focus and pointer were left unchanged"
        ));
    }
    Ok(())
}

fn status() -> Result<(), String> {
    let client = hyprland()?;
    let windows = client.windows().map_err(|error| error.to_string())?;
    let focused = windows.iter().find(|window| window.focused);
    println!("Gridlock CLI 0.1.0");
    println!("Compositor: Hyprland ({} windows)", windows.len());
    match focused {
        Some(window) => println!("Focused: {}", safe_title(&window.title)),
        None => println!("Focused: none"),
    }
    Ok(())
}

fn safe_title(value: &str) -> String {
    value
        .chars()
        .filter(|character| !character.is_control())
        .take(200)
        .collect()
}

fn config_path() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(path).join("gridlock/config.toml"));
    }
    let home = std::env::var_os("HOME").ok_or("HOME is not set")?;
    Ok(PathBuf::from(home).join(".config/gridlock/config.toml"))
}

fn read_config() -> Result<Config, String> {
    let path = config_path()?;
    match std::fs::read_to_string(&path) {
        Ok(source) => {
            Config::parse(&source).map_err(|error| format!("{}: {error}", path.display()))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(error) => Err(format!("{}: {error}", path.display())),
    }
}

fn validate_config() -> Result<(), String> {
    let path = config_path()?;
    match std::fs::read_to_string(&path) {
        Ok(source) => {
            Config::parse(&source).map_err(|error| format!("{}: {error}", path.display()))?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("No config exists at {}; defaults are valid", path.display());
            return Ok(());
        }
        Err(error) => return Err(format!("{}: {error}", path.display())),
    }
    println!("Configuration is valid");
    Ok(())
}
