use gridlock_core::{Direction, Rect, Window};
use serde::Deserialize;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

const MAX_REPLY_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub enum Error {
    MissingRuntime,
    MissingInstance,
    Io(std::io::Error),
    Protocol(String),
    Json(serde_json::Error),
    Command(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRuntime => write!(
                f,
                "XDG_RUNTIME_DIR is not set; cannot locate the Hyprland IPC socket"
            ),
            Self::MissingInstance => write!(
                f,
                "HYPRLAND_INSTANCE_SIGNATURE is not set; Gridlock needs a running Hyprland session"
            ),
            Self::Io(error) => write!(f, "Hyprland IPC I/O error: {error}"),
            Self::Protocol(error) => write!(f, "Hyprland IPC protocol error: {error}"),
            Self::Json(error) => write!(f, "invalid Hyprland JSON response: {error}"),
            Self::Command(error) => write!(f, "Hyprland rejected command: {error}"),
        }
    }
}
impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

#[derive(Debug, Clone)]
pub struct HyprlandClient {
    socket: PathBuf,
}

impl HyprlandClient {
    pub fn from_env() -> Result<Self, Error> {
        let runtime = std::env::var_os("XDG_RUNTIME_DIR").ok_or(Error::MissingRuntime)?;
        let instance =
            std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").ok_or(Error::MissingInstance)?;
        Ok(Self::at(
            PathBuf::from(runtime)
                .join("hypr")
                .join(instance)
                .join(".socket.sock"),
        ))
    }

    pub fn at(socket: impl AsRef<Path>) -> Self {
        Self {
            socket: socket.as_ref().to_path_buf(),
        }
    }

    fn request(&self, message: &str) -> Result<String, Error> {
        let mut stream = UnixStream::connect(&self.socket)?;
        stream.set_read_timeout(Some(Duration::from_secs(3)))?;
        stream.set_write_timeout(Some(Duration::from_secs(3)))?;
        stream.write_all(message.as_bytes())?;
        stream.shutdown(std::net::Shutdown::Write)?;
        let mut response = Vec::new();
        stream
            .take((MAX_REPLY_SIZE + 1) as u64)
            .read_to_end(&mut response)?;
        if response.len() > MAX_REPLY_SIZE {
            return Err(Error::Protocol("reply exceeded size limit".into()));
        }
        String::from_utf8(response)
            .map_err(|e| Error::Protocol(format!("reply was not UTF-8: {e}")))
    }

    pub fn windows(&self) -> Result<Vec<Window>, Error> {
        let reply = self.request("j/clients")?;
        let clients: Vec<Client> = serde_json::from_str(&reply)?;
        Ok(clients
            .into_iter()
            .filter(|c| c.mapped && !c.hidden)
            .map(|c| Window {
                id: address_id(&c.address),
                title: c.title,
                rect: Rect {
                    x: c.at[0],
                    y: c.at[1],
                    width: c.size[0],
                    height: c.size[1],
                },
                focused: c.focus_history_id == 0,
                floating: c.floating,
            })
            .collect())
    }

    pub fn focus_direction(
        &self,
        direction: Direction,
        exclude_floating: bool,
        center_pointer: bool,
    ) -> Result<bool, Error> {
        let windows = self.windows()?;
        let Some(target) =
            gridlock_core::directional_target(&windows, direction, exclude_floating).cloned()
        else {
            return Ok(false);
        };
        self.dispatch(&format!("focuswindow address:0x{:x}", target.id))?;
        if center_pointer {
            // Hyprland's movecursor is an IPC dispatcher. Its coordinate semantics
            // and availability must be validated against each supported FreeBSD build.
            let (x, y) = target.rect.center();
            self.dispatch(&format!("movecursor {x} {y}"))?;
        }
        Ok(true)
    }

    pub fn move_focused(&self, direction: Direction) -> Result<(), Error> {
        let d = match direction {
            Direction::Left => "l",
            Direction::Right => "r",
            Direction::Up => "u",
            Direction::Down => "d",
        };
        self.dispatch(&format!("movewindow {d}"))
    }
    pub fn switch_workspace(&self, number: u8) -> Result<(), Error> {
        validate_workspace(number)?;
        self.dispatch(&format!("workspace {number}"))
    }
    pub fn move_focused_to_workspace(&self, number: u8) -> Result<(), Error> {
        validate_workspace(number)?;
        self.dispatch(&format!("movetoworkspace {number}"))
    }
    pub fn close_focused(&self) -> Result<(), Error> {
        self.dispatch("killactive")
    }
    pub fn toggle_fullscreen(&self) -> Result<(), Error> {
        self.dispatch("fullscreen")
    }
    pub fn toggle_floating(&self) -> Result<(), Error> {
        self.dispatch("togglefloating")
    }

    fn dispatch(&self, command: &str) -> Result<(), Error> {
        let reply = self.request(&format!("/dispatch {command}"))?;
        if reply.trim() == "ok" {
            Ok(())
        } else {
            Err(Error::Command(reply.trim().to_owned()))
        }
    }
}

fn validate_workspace(number: u8) -> Result<(), Error> {
    if (1..=9).contains(&number) {
        Ok(())
    } else {
        Err(Error::Command(
            "workspace number must be from 1 through 9".into(),
        ))
    }
}

fn address_id(address: &str) -> i64 {
    let value = address.strip_prefix("0x").unwrap_or(address);
    i64::from_str_radix(value, 16).unwrap_or(0)
}

#[derive(Deserialize)]
struct Client {
    address: String,
    title: String,
    at: [i64; 2],
    size: [u64; 2],
    floating: bool,
    mapped: bool,
    hidden: bool,
    #[serde(rename = "focusHistoryID")]
    focus_history_id: i64,
}
