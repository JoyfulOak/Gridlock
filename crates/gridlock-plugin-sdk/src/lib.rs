use serde::{Deserialize, Serialize};
use std::path::Path;

pub const PLUGIN_API_VERSION: u32 = 1;
pub const MANIFEST_FILE: &str = "plugin.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub api: ApiRange,
    pub runtime: PluginRuntime,
    #[serde(default)]
    pub entrypoint: Option<String>,
    #[serde(default)]
    pub architectures: Vec<String>,
    #[serde(default)]
    pub required_desktop_version: Option<String>,
    #[serde(default)]
    pub dependencies: Vec<PluginDependency>,
    #[serde(default)]
    pub permissions: Vec<Permission>,
    #[serde(default)]
    pub config_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApiRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginRuntime {
    Data,
    Executable,
    Wasm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PluginDependency {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    ReadCompositor,
    ControlCompositor,
    ReadNotifications,
    SendNotifications,
    ReadFiles,
    WriteFiles,
    ExecuteProcesses,
    Network,
    SystemAdministration,
}

impl PluginManifest {
    pub fn parse(source: &str) -> Result<Self, String> {
        let manifest: Self = toml::from_str(source).map_err(|error| error.to_string())?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<(), String> {
        if !valid_id(&self.id) {
            return Err("plugin id must be reverse-domain-like lowercase ASCII (letters, digits, dots, and hyphens)".into());
        }
        if self.name.trim().is_empty() || self.version.trim().is_empty() {
            return Err("plugin name and version are required".into());
        }
        if self.api.min > self.api.max || self.api.min > PLUGIN_API_VERSION || self.api.max < PLUGIN_API_VERSION {
            return Err(format!("plugin API range {}..={} does not support Gridlock Plugin SDK API {}", self.api.min, self.api.max, PLUGIN_API_VERSION));
        }
        match self.runtime {
            PluginRuntime::Data if self.entrypoint.is_some() => {
                return Err("data-only plugins must not declare an executable entrypoint".into());
            }
            PluginRuntime::Executable | PluginRuntime::Wasm if self.entrypoint.is_none() => {
                return Err("executable and WASM plugins require an entrypoint".into());
            }
            _ => {}
        }
        if let Some(entrypoint) = &self.entrypoint {
            let path = Path::new(entrypoint);
            if path.is_absolute() || path.components().any(|part| !matches!(part, std::path::Component::Normal(_))) {
                return Err("entrypoint must be a relative path inside the plugin package".into());
            }
        }
        for architecture in &self.architectures {
            if !matches!(architecture.as_str(), "amd64" | "aarch64") {
                return Err(format!("unsupported architecture {architecture:?}; expected amd64 or aarch64"));
            }
        }
        for dependency in &self.dependencies {
            if !valid_id(&dependency.id) || dependency.version.trim().is_empty() {
                return Err(format!("invalid dependency declaration for {:?}", dependency.id));
            }
            if dependency.id == self.id {
                return Err("a plugin cannot depend on itself".into());
            }
        }
        Ok(())
    }
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.split('.').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = r#"
id = "org.example.clock"
name = "Clock Widget"
version = "0.1.0"
runtime = "data"

[api]
min = 1
max = 1
"#;

    #[test]
    fn accepts_current_data_plugin_manifest() {
        assert_eq!(PluginManifest::parse(VALID).unwrap().runtime, PluginRuntime::Data);
    }

    #[test]
    fn rejects_entrypoint_escape() {
        let source = VALID.replace("runtime = \"data\"", "runtime = \"executable\"\nentrypoint = \"../run.sh\"");
        assert!(PluginManifest::parse(&source).unwrap_err().contains("relative path"));
    }

    #[test]
    fn rejects_incompatible_api_range() {
        let source = VALID.replace("min = 1", "min = 2").replace("max = 1", "max = 2");
        assert!(PluginManifest::parse(&source).unwrap_err().contains("does not support"));
    }
}
