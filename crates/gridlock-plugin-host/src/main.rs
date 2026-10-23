use gridlock_plugin_sdk::{Permission, PluginManifest, MANIFEST_FILE};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    if let Err(error) = run(std::env::args().skip(1).collect()) {
        eprintln!("gridlock-plugin-host: {error}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    match args.as_slice() {
        [command] if command == "list" => list_plugins(),
        [command, path] if command == "inspect" => inspect(Path::new(path)),
        [command, id] if command == "enable" => set_enabled(id, true),
        [command, id] if command == "disable" => set_enabled(id, false),
        [command, path] if command == "install" => install(Path::new(path)),
        [command, id] if command == "remove" => remove_plugin(id),
        _ => Err("usage: gridlock-plugin-host list | inspect <directory> | install <directory> | enable <id> | disable <id> | remove <id>".into()),
    }
}

fn plugin_root() -> Result<PathBuf, String> {
    let data = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
        .ok_or("XDG_DATA_HOME and HOME are unset")?;
    Ok(data.join("gridlock/plugins"))
}

fn load_manifest(directory: &Path) -> Result<PluginManifest, String> {
    let path = directory.join(MANIFEST_FILE);
    let source =
        fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    PluginManifest::parse(&source).map_err(|error| format!("{}: {error}", path.display()))
}

fn inspect(directory: &Path) -> Result<(), String> {
    let manifest = load_manifest(directory)?;
    println!("{} {} ({})", manifest.name, manifest.version, manifest.id);
    println!(
        "Runtime: {:?}; API {}..={}",
        manifest.runtime, manifest.api.min, manifest.api.max
    );
    println!(
        "Architectures: {}",
        if manifest.architectures.is_empty() {
            "all supported".into()
        } else {
            manifest.architectures.join(", ")
        }
    );
    println!("Permissions: {}", permission_list(&manifest.permissions));
    if let Some(entrypoint) = manifest.entrypoint {
        println!("Entrypoint: {entrypoint}");
    }
    Ok(())
}

fn list_plugins() -> Result<(), String> {
    let root = plugin_root()?;
    let entries = match fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            println!("No plugins installed");
            return Ok(());
        }
        Err(error) => return Err(format!("{}: {error}", root.display())),
    };
    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        if !entry
            .file_type()
            .map_err(|error| error.to_string())?
            .is_dir()
        {
            continue;
        }
        match load_manifest(&entry.path()) {
            Ok(manifest) => {
                let enabled = entry.path().join("enabled").is_file();
                println!(
                    "{} {} [{}] {}",
                    manifest.id,
                    manifest.version,
                    if enabled { "enabled" } else { "disabled" },
                    manifest.name
                );
            }
            Err(error) => eprintln!("Skipping invalid plugin: {error}"),
        }
    }
    Ok(())
}

fn install(source: &Path) -> Result<(), String> {
    let manifest = load_manifest(source)?;
    println!(
        "Plugin: {} {} ({})",
        manifest.name, manifest.version, manifest.id
    );
    println!(
        "Requested permissions: {}",
        permission_list(&manifest.permissions)
    );
    println!(
        "Installation copies files only. It does not run scripts, enable, or execute the plugin."
    );
    print!("Type install to approve copying this plugin: ");
    io::stdout().flush().map_err(|error| error.to_string())?;
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|error| error.to_string())?;
    if answer.trim() != "install" {
        return Err("installation cancelled".into());
    }

    let root = plugin_root()?;
    fs::create_dir_all(&root).map_err(|error| format!("{}: {error}", root.display()))?;
    let destination = root.join(&manifest.id);
    if destination.exists() {
        return Err(format!("plugin {} is already installed", manifest.id));
    }
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let stage = root.join(format!(".install-{}-{nonce}", std::process::id()));
    if let Err(error) = copy_tree(source, &stage) {
        let _ = fs::remove_dir_all(&stage);
        return Err(error);
    }
    if let Err(error) = fs::rename(&stage, &destination) {
        let _ = fs::remove_dir_all(&stage);
        return Err(format!("could not finalize plugin install: {error}"));
    }
    println!("Installed {} as disabled", manifest.id);
    Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    let metadata =
        fs::symlink_metadata(source).map_err(|error| format!("{}: {error}", source.display()))?;
    if metadata.file_type().is_symlink() {
        return Err(format!(
            "refusing symlink in plugin package: {}",
            source.display()
        ));
    }
    if metadata.is_dir() {
        fs::create_dir(destination)
            .map_err(|error| format!("{}: {error}", destination.display()))?;
        for entry in fs::read_dir(source).map_err(|error| error.to_string())? {
            let entry = entry.map_err(|error| error.to_string())?;
            copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
        }
    } else if metadata.is_file() {
        fs::copy(source, destination).map_err(|error| format!("{}: {error}", source.display()))?;
    } else {
        return Err(format!(
            "refusing special file in plugin package: {}",
            source.display()
        ));
    }
    Ok(())
}

fn set_enabled(id: &str, enabled: bool) -> Result<(), String> {
    if id.contains('/') || id.contains('\\') || id == "." || id == ".." {
        return Err("invalid plugin id".into());
    }
    let directory = plugin_root()?.join(id);
    let manifest = load_manifest(&directory)?;
    if manifest.id != id {
        return Err(format!(
            "installed directory {id:?} contains plugin {:?}",
            manifest.id
        ));
    }
    let marker = directory.join("enabled");
    if enabled {
        fs::write(&marker, b"enabled\n").map_err(|error| error.to_string())?;
        println!("Enabled {id}. Execution support is not implemented yet.");
    } else {
        match fs::remove_file(&marker) {
            Ok(()) => println!("Disabled {id}"),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                println!("{id} is already disabled")
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}

fn remove_plugin(id: &str) -> Result<(), String> {
    if id.contains('/') || id.contains('\\') || id == "." || id == ".." {
        return Err("invalid plugin id".into());
    }
    let directory = plugin_root()?.join(id);
    let manifest = load_manifest(&directory)?;
    if manifest.id != id {
        return Err(format!(
            "installed directory {id:?} contains plugin {:?}",
            manifest.id
        ));
    }
    print!(
        "Remove installed plugin {}? Type remove to confirm: ",
        manifest.id
    );
    io::stdout().flush().map_err(|error| error.to_string())?;
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .map_err(|error| error.to_string())?;
    if answer.trim() != "remove" {
        return Err("removal cancelled".into());
    }
    fs::remove_dir_all(&directory).map_err(|error| error.to_string())?;
    println!("Removed installed files for {}", manifest.id);
    Ok(())
}

fn permission_list(permissions: &[Permission]) -> String {
    if permissions.is_empty() {
        "none".into()
    } else {
        permissions
            .iter()
            .map(|permission| format!("{permission:?}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
