//! # Path
//!
//! System path helpers

use std::path::{Path, PathBuf};

use super::file as file_helper;

/// Get tuifeed configuration directory path.
/// Returns None, if it's not possible to get it
pub fn init_config_dir() -> Result<Option<PathBuf>, String> {
    // Get file
    #[cfg(not(test))]
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = dirs::config_dir();
    }
    #[cfg(test)]
    lazy_static! {
        static ref CONF_DIR: Option<PathBuf> = Some(std::env::temp_dir());
    }
    if CONF_DIR.is_some() {
        // Get path of bookmarks
        let mut p: PathBuf = CONF_DIR.as_ref().unwrap().clone();
        // Append tuifeed dir
        p.push("tuifeed/");
        // If directory doesn't exist, create it
        if p.exists() {
            return Ok(Some(p));
        }
        match std::fs::create_dir(p.as_path()) {
            Ok(_) => Ok(Some(p)),
            Err(err) => Err(err.to_string()),
        }
    } else {
        Ok(None)
    }
}

/// Returns path for config file.
/// If the file doesn't exist, it will initialize it
pub fn get_config_file(config_dir: &Path) -> Result<PathBuf, String> {
    let cfg_file = config_dir.join("config.toml");
    // Check if exists
    if !cfg_file.exists() {
        init_config_file(cfg_file.as_path())?
    }
    Ok(cfg_file)
}

/// Initialize configuration file
fn init_config_file(p: &Path) -> Result<(), String> {
    file_helper::write_file(
        p,
        r##"[sources]
# Write here the sources you want to get the feed from following the example below
# New_York_Times = "https://rss.nytimes.com/services/xml/rss/nyt/World.xml"
"##,
    )
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {

    use std::fs::{File, OpenOptions};
    use std::io::Write;

    use pretty_assertions::assert_eq;
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn should_get_config_dir() {
        // Create and get conf_dir
        let conf_dir: PathBuf = init_config_dir().ok().unwrap().unwrap();
        // Remove dir
        assert!(std::fs::remove_dir_all(conf_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn should_fail_getting_config_dir() {
        let mut conf_dir: PathBuf = std::env::temp_dir();
        conf_dir.push("tuifeed");
        // Create file
        let mut f: File = OpenOptions::new()
            .create(true)
            .write(true)
            .open(conf_dir.as_path())
            .ok()
            .unwrap();
        // Write
        assert!(writeln!(f, "Hello world!").is_ok());
        // Drop file
        drop(f);
        // Get config dir (will fail)
        assert!(init_config_dir().is_err());
        // Remove file
        assert!(std::fs::remove_file(conf_dir.as_path()).is_ok());
    }

    #[test]
    #[serial]
    fn should_get_config_file() {
        let conf_dir: PathBuf = init_config_dir().ok().unwrap().unwrap();
        let cfg_file = get_config_file(conf_dir.as_path()).ok().unwrap();
        assert_eq!(
            format!("{}", cfg_file.display()),
            format!("{}config.toml", conf_dir.display())
        );
        assert!(std::fs::remove_dir_all(conf_dir.as_path()).is_ok());
    }
}
