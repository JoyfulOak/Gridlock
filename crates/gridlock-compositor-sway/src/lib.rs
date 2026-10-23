use gridlock_core::{Direction, Rect, Window};
use serde::Deserialize;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

const IPC_MAGIC: &[u8; 6] = b"i3-ipc";
const IPC_COMMAND: u32 = 0;
const IPC_GET_TREE: u32 = 4;
const MAX_REPLY_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub enum Error {
    MissingSocket,
    Io(std::io::Error),
    Protocol(String),
    Json(serde_json::Error),
    Command(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSocket => write!(formatter, "SWAYSOCK is not set; Gridlock needs a running Sway session"),
            Self::Io(error) => write!(formatter, "Sway IPC I/O error: {error}"),
            Self::Protocol(error) => write!(formatter, "Sway IPC protocol error: {error}"),
            Self::Json(error) => write!(formatter, "invalid Sway JSON response: {error}"),
            Self::Command(error) => write!(formatter, "Sway rejected command: {error}"),
        }
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error { fn from(error: std::io::Error) -> Self { Self::Io(error) } }
impl From<serde_json::Error> for Error { fn from(error: serde_json::Error) -> Self { Self::Json(error) } }

#[derive(Debug, Clone)]
pub struct SwayClient {
    socket: PathBuf,
}

impl SwayClient {
    pub fn from_env() -> Result<Self, Error> {
        let socket = std::env::var_os("SWAYSOCK").ok_or(Error::MissingSocket)?;
        Ok(Self { socket: PathBuf::from(socket) })
    }

    pub fn at(socket: impl AsRef<Path>) -> Self {
        Self { socket: socket.as_ref().to_path_buf() }
    }

    fn request(&self, kind: u32, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let mut stream = UnixStream::connect(&self.socket)?;
        stream.set_read_timeout(Some(Duration::from_secs(3)))?;
        stream.set_write_timeout(Some(Duration::from_secs(3)))?;
        stream.write_all(IPC_MAGIC)?;
        stream.write_all(&(payload.len() as u32).to_ne_bytes())?;
        stream.write_all(&kind.to_ne_bytes())?;
        stream.write_all(payload)?;

        let mut header = [0_u8; 14];
        stream.read_exact(&mut header)?;
        if &header[..6] != IPC_MAGIC {
            return Err(Error::Protocol("bad response magic".into()));
        }
        let size = u32::from_ne_bytes(header[6..10].try_into().unwrap()) as usize;
        let response_kind = u32::from_ne_bytes(header[10..14].try_into().unwrap());
        if response_kind != kind {
            return Err(Error::Protocol(format!("reply type {response_kind} did not match request {kind}")));
        }
        if size > MAX_REPLY_SIZE {
            return Err(Error::Protocol(format!("reply exceeded {MAX_REPLY_SIZE} bytes")));
        }
        let mut response = vec![0; size];
        stream.read_exact(&mut response)?;
        Ok(response)
    }

    pub fn windows(&self) -> Result<Vec<Window>, Error> {
        let bytes = self.request(IPC_GET_TREE, &[])?;
        let tree: Node = serde_json::from_slice(&bytes)?;
        let mut windows = Vec::new();
        collect_windows(&tree, false, true, &mut windows);
        Ok(windows)
    }

    pub fn focus_window(&self, id: i64) -> Result<(), Error> {
        // The identifier is parsed from Sway's integer tree field, so it cannot
        // inject additional command syntax into this selector.
        self.command(&format!("[con_id={id}] focus"))
    }

    pub fn switch_workspace(&self, number: u8) -> Result<(), Error> {
        if !(1..=9).contains(&number) {
            return Err(Error::Command("workspace number must be from 1 through 9".into()));
        }
        self.command(&format!("workspace number {number}"))
    }

    pub fn move_focused_to_workspace(&self, number: u8) -> Result<(), Error> {
        if !(1..=9).contains(&number) {
            return Err(Error::Command("workspace number must be from 1 through 9".into()));
        }
        self.command(&format!("move container to workspace number {number}"))
    }

    pub fn move_focused(&self, direction: Direction) -> Result<(), Error> {
        let direction = match direction {
            Direction::Left => "left",
            Direction::Right => "right",
            Direction::Up => "up",
            Direction::Down => "down",
        };
        self.command(&format!("move {direction}"))
    }

    pub fn close_focused(&self) -> Result<(), Error> { self.command("kill") }
    pub fn toggle_fullscreen(&self) -> Result<(), Error> { self.command("fullscreen toggle") }
    pub fn toggle_floating(&self) -> Result<(), Error> { self.command("floating toggle") }

    pub fn center_cursor(&self, rect: Rect) -> Result<(), Error> {
        let (x, y) = rect.center();
        // Sway currently documents this absolute cursor command as deprecated.
        // Keep the call isolated so the backend can be replaced after target
        // package testing identifies the supported virtual-pointer API.
        self.command(&format!("seat seat0 cursor set {x} {y}"))
    }

    pub fn command(&self, command: &str) -> Result<(), Error> {
        let response = self.request(IPC_COMMAND, command.as_bytes())?;
        let results: Vec<CommandResult> = serde_json::from_slice(&response)?;
        if let Some(failure) = results.iter().find(|result| !result.success) {
            return Err(Error::Command(failure.error.clone().unwrap_or_else(|| "command failed".into())));
        }
        Ok(())
    }

    pub fn focus_direction(&self, direction: Direction, exclude_floating: bool, center_pointer: bool) -> Result<bool, Error> {
        let windows = self.windows()?;
        let Some(target) = gridlock_core::directional_target(&windows, direction, exclude_floating).cloned() else {
            return Ok(false);
        };
        self.focus_window(target.id)?;
        if center_pointer {
            self.center_cursor(target.rect)?;
        }
        Ok(true)
    }
}

#[derive(Debug, Deserialize)]
struct CommandResult {
    success: bool,
    error: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct Node {
    id: Option<i64>,
    #[serde(rename = "type")]
    node_type: Option<String>,
    name: Option<String>,
    app_id: Option<String>,
    window: Option<i64>,
    focused: Option<bool>,
    visible: Option<bool>,
    rect: Option<Rect>,
    nodes: Option<Vec<Node>>,
    floating_nodes: Option<Vec<Node>>,
    shell: Option<String>,
}

fn collect_windows(node: &Node, inherited_floating: bool, inherited_visible: bool, output: &mut Vec<Window>) {
    let floating = inherited_floating || node.shell.as_deref() == Some("floating")
        || node.node_type.as_deref() == Some("floating_con");
    let visible = inherited_visible && node.visible.unwrap_or(true);
    let children = node.nodes.as_deref().unwrap_or_default();
    let floating_children = node.floating_nodes.as_deref().unwrap_or_default();
    let has_children = !children.is_empty() || !floating_children.is_empty();
    let is_window = node.node_type.as_deref() == Some("con")
        && !has_children
        && (node.window.is_some() || node.app_id.is_some());
    if is_window && visible {
        if let (Some(id), Some(rect)) = (node.id, node.rect) {
            output.push(Window {
                id,
                title: node.name.clone().unwrap_or_else(|| "Untitled".into()),
                rect,
                focused: node.focused.unwrap_or(false),
                floating,
            });
        }
    }
    for child in children {
        collect_windows(child, floating, visible, output);
    }
    for child in floating_children {
        collect_windows(child, true, visible, output);
    }
}
