use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

pub const CONFIG_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub pointer_follow_focus: bool,
    pub exclude_floating_from_directional_focus: bool,
    pub terminal: String,
    pub launcher: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            pointer_follow_focus: true,
            exclude_floating_from_directional_focus: false,
            terminal: "xterm".into(),
            launcher: "gridlock-shell".into(),
        }
    }
}

impl Config {
    pub fn parse(source: &str) -> Result<Self, String> {
        let config: Self = toml::from_str(source).map_err(|error| error.to_string())?;
        if config.version != CONFIG_VERSION {
            return Err(format!(
                "unsupported config version {}; expected {}",
                config.version, CONFIG_VERSION
            ));
        }
        if config.terminal.trim().is_empty() || config.launcher.trim().is_empty() {
            return Err("terminal and launcher commands must not be empty".into());
        }
        Ok(config)
    }

    pub fn to_toml(&self) -> Result<String, String> {
        toml::to_string_pretty(self).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(format!(
                "unknown direction {value:?}; use left, right, up, or down"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: u64,
    pub height: u64,
}

impl Rect {
    pub fn center(self) -> (i64, i64) {
        (
            self.x.saturating_add((self.width / 2) as i64),
            self.y.saturating_add((self.height / 2) as i64),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub id: i64,
    pub title: String,
    pub rect: Rect,
    pub focused: bool,
    pub floating: bool,
}

pub fn directional_target(
    windows: &[Window],
    direction: Direction,
    exclude_floating: bool,
) -> Option<&Window> {
    let focused = windows.iter().find(|window| window.focused)?;
    let (fx, fy) = focused.rect.center();

    windows
        .iter()
        .filter(|candidate| candidate.id != focused.id)
        .filter(|candidate| !exclude_floating || !candidate.floating)
        .filter_map(|candidate| {
            let (x, y) = candidate.rect.center();
            let dx = x.saturating_sub(fx);
            let dy = y.saturating_sub(fy);
            let (primary, secondary, is_forward) = match direction {
                Direction::Left => (dx.saturating_abs(), dy.saturating_abs(), dx < 0),
                Direction::Right => (dx.saturating_abs(), dy.saturating_abs(), dx > 0),
                Direction::Up => (dy.saturating_abs(), dx.saturating_abs(), dy < 0),
                Direction::Down => (dy.saturating_abs(), dx.saturating_abs(), dy > 0),
            };
            if !is_forward {
                return None;
            }
            // Prefer a nearby window in the requested direction, with a modest
            // penalty for candidates far off the same row/column.
            let score = (primary as u128)
                .saturating_mul(primary as u128)
                .saturating_add(
                    (secondary as u128)
                        .saturating_mul(secondary as u128)
                        .saturating_mul(2),
                );
            Some((candidate, score, primary, secondary))
        })
        .min_by(|left, right| {
            left.1
                .cmp(&right.1)
                .then_with(|| left.2.cmp(&right.2))
                .then_with(|| left.3.cmp(&right.3))
                .then(Ordering::Equal)
        })
        .map(|(candidate, _, _, _)| candidate)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window(id: i64, x: i64, y: i64, focused: bool) -> Window {
        Window {
            id,
            title: format!("window-{id}"),
            rect: Rect {
                x,
                y,
                width: 100,
                height: 100,
            },
            focused,
            floating: false,
        }
    }

    #[test]
    fn picks_nearest_window_in_direction() {
        let windows = [
            window(1, 100, 100, true),
            window(2, 300, 100, false),
            window(3, 500, 100, false),
        ];
        assert_eq!(
            directional_target(&windows, Direction::Right, false)
                .unwrap()
                .id,
            2
        );
    }

    #[test]
    fn does_not_wrap_when_no_window_is_in_direction() {
        let windows = [window(1, 100, 100, true), window(2, 0, 100, false)];
        assert!(directional_target(&windows, Direction::Right, false).is_none());
    }

    #[test]
    fn can_exclude_floating_windows() {
        let mut floating = window(2, 300, 100, false);
        floating.floating = true;
        let windows = [
            window(1, 100, 100, true),
            floating,
            window(3, 100, 300, false),
        ];
        assert_eq!(directional_target(&windows, Direction::Right, true), None);
        assert_eq!(
            directional_target(&windows, Direction::Down, true)
                .unwrap()
                .id,
            3
        );
    }

    #[test]
    fn config_requires_current_schema_version() {
        assert!(Config::parse("version = 99")
            .unwrap_err()
            .contains("unsupported config version"));
    }
}
