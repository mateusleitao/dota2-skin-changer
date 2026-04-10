use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GamePathError {
    #[error("Dota 2 installation not found")]
    NotFound,
    #[error("Steam installation not found in registry")]
    SteamNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid library folders config")]
    InvalidLibraryFolders,
}

/// Trait for abstracting game path resolution (mockable for macOS testing)
pub trait GamePathResolver {
    fn find_dota2_path(&self) -> Result<PathBuf, GamePathError>;
    fn find_steam_path(&self) -> Result<PathBuf, GamePathError>;
}

/// Windows-specific resolver that reads the registry and Steam library folders
pub struct WindowsGamePathResolver;

impl GamePathResolver for WindowsGamePathResolver {
    fn find_steam_path(&self) -> Result<PathBuf, GamePathError> {
        #[cfg(target_os = "windows")]
        {
            use winreg::enums::*;
            use winreg::RegKey;
            let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
            let steam_key = hklm
                .open_subkey("SOFTWARE\\Valve\\Steam")
                .or_else(|_| hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam"))
                .map_err(|_| GamePathError::SteamNotFound)?;
            let install_path: String = steam_key
                .get_value("InstallPath")
                .map_err(|_| GamePathError::SteamNotFound)?;
            Ok(PathBuf::from(install_path))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Err(GamePathError::SteamNotFound)
        }
    }

    fn find_dota2_path(&self) -> Result<PathBuf, GamePathError> {
        let steam_path = self.find_steam_path()?;
        let library_folders = steam_path.join("steamapps").join("libraryfolders.vdf");

        if library_folders.exists() {
            let content = std::fs::read_to_string(&library_folders)?;
            if let Some(path) = find_dota2_in_library_folders(&content) {
                return Ok(path);
            }
        }

        let default_path = steam_path
            .join("steamapps")
            .join("common")
            .join("dota 2 beta");
        if default_path.exists() {
            return Ok(default_path);
        }

        Err(GamePathError::NotFound)
    }
}

/// Parse `libraryfolders.vdf` to find the Dota 2 installation across multiple drives
pub fn find_dota2_in_library_folders(content: &str) -> Option<PathBuf> {
    let mut current_path: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("\"path\"") {
            if let Some(path) = extract_vdf_value(trimmed) {
                current_path = Some(path);
            }
        }

        if trimmed.contains("\"570\"") {
            if let Some(ref lib_path) = current_path {
                let dota_path = PathBuf::from(lib_path)
                    .join("steamapps")
                    .join("common")
                    .join("dota 2 beta");
                if dota_path.exists() {
                    return Some(dota_path);
                }
            }
        }
    }

    None
}

fn extract_vdf_value(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split('"').collect();
    if parts.len() >= 4 {
        Some(parts[3].replace("\\\\", "\\"))
    } else {
        None
    }
}

/// Validate that a path contains a valid Dota 2 installation
pub fn validate_dota2_path(path: &Path) -> bool {
    let vpk = path.join("game").join("dota").join("pak01_dir.vpk");
    let bin = path.join("game").join("bin").join("win64");
    vpk.exists() && bin.exists()
}

/// Mock resolver for testing on macOS
pub struct MockGamePathResolver {
    pub dota2_path: Option<PathBuf>,
    pub steam_path: Option<PathBuf>,
}

impl GamePathResolver for MockGamePathResolver {
    fn find_steam_path(&self) -> Result<PathBuf, GamePathError> {
        self.steam_path.clone().ok_or(GamePathError::SteamNotFound)
    }

    fn find_dota2_path(&self) -> Result<PathBuf, GamePathError> {
        self.dota2_path.clone().ok_or(GamePathError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_vdf_value() {
        let line = r#"		"path"		"D:\\SteamLibrary""#;
        assert_eq!(
            extract_vdf_value(line),
            Some("D:\\SteamLibrary".to_string())
        );
    }

    #[test]
    fn test_extract_vdf_value_simple() {
        let line = r#""path"		"C:\\Program Files (x86)\\Steam""#;
        assert_eq!(
            extract_vdf_value(line),
            Some("C:\\Program Files (x86)\\Steam".to_string())
        );
    }

    #[test]
    fn test_mock_resolver_not_found() {
        let resolver = MockGamePathResolver {
            dota2_path: None,
            steam_path: None,
        };
        assert!(resolver.find_dota2_path().is_err());
        assert!(resolver.find_steam_path().is_err());
    }

    #[test]
    fn test_mock_resolver_found() {
        let resolver = MockGamePathResolver {
            dota2_path: Some(PathBuf::from("/mock/dota2")),
            steam_path: Some(PathBuf::from("/mock/steam")),
        };
        assert_eq!(
            resolver.find_dota2_path().unwrap(),
            PathBuf::from("/mock/dota2")
        );
    }

    #[test]
    fn test_find_dota2_in_library_folders_no_match() {
        let content = r#"
"libraryfolders"
{
    "0"
    {
        "path"      "C:\\Program Files (x86)\\Steam"
        "apps"
        {
            "228980"    "123456"
        }
    }
}
"#;
        assert!(find_dota2_in_library_folders(content).is_none());
    }
}
