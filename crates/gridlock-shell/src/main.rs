use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// This first shell milestone is a compositor-independent status provider. It
// is intentionally separate from the eventual GTK4/layer-shell UI until that
// surface stack is confirmed on the selected FreeBSD target.
fn main() {
    let interval = std::env::var("GRIDLOCK_STATUS_INTERVAL")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(2);
    loop {
        match render_status() {
            Ok(line) => {
                println!("{line}");
                let _ = io::stdout().flush();
            }
            Err(error) => {
                eprintln!("gridlock-shell: {error}");
                println!("Gridlock | Hyprland unavailable");
            }
        }
        thread::sleep(Duration::from_secs(interval.max(1)));
    }
}

fn render_status() -> Result<String, String> {
    let client = gridlock_compositor_hyprland::HyprlandClient::from_env()
        .map_err(|error| error.to_string())?;
    let windows = client.windows().map_err(|error| error.to_string())?;
    let title: String = windows
        .iter()
        .find(|window| window.focused)
        .map(|window| {
            window
                .title
                .chars()
                .filter(|character| !character.is_control())
                .take(200)
                .collect()
        })
        .unwrap_or_else(|| "Desktop".into());
    let time = std::process::Command::new("date")
        .arg("+%H:%M")
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_default();
    Ok(format!("Gridlock  |  {title}  |  {time}"))
}
