use crate::backup::{self, BackupError};
use crate::item_db;
use crate::models::{HookStatus, InstallResult};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Backup error: {0}")]
    Backup(#[from] BackupError),
    #[error("Item database error: {0}")]
    ItemDb(String),
    #[error("Hook DLL not found in application bundle")]
    HookDllNotFound,
    #[error("Game path not provided")]
    NoGamePath,
    #[error("Original DLL not found at {0}")]
    OriginalDllNotFound(String),
}

const ORIGINAL_DLL_NAME: &str = "steam_api64.dll";
const RENAMED_ORIGINAL: &str = "steam_api64_o.dll";
const ITEM_DB_NAME: &str = "item_db.bin";

/// Get the path where the steam_api64.dll lives in a Dota 2 installation
pub fn dll_dir(game_path: &Path) -> PathBuf {
    game_path.join("game").join("bin").join("win64")
}

/// Get the path to pak01_dir.vpk
pub fn vpk_path(game_path: &Path) -> PathBuf {
    game_path.join("game").join("dota").join("pak01_dir.vpk")
}

/// Install the hook: backup original, rename, copy proxy, generate item_db
pub fn install(
    game_path: &Path,
    hook_dll_bytes: &[u8],
    app_version: &str,
) -> Result<InstallResult, InstallerError> {
    let dir = dll_dir(game_path);
    let original = dir.join(ORIGINAL_DLL_NAME);
    let renamed = dir.join(RENAMED_ORIGINAL);

    if !original.exists() {
        return Err(InstallerError::OriginalDllNotFound(
            original.display().to_string(),
        ));
    }

    let backup_dir = backup::default_backup_dir(game_path);
    let backup_info = backup::create_backup(&original, &backup_dir, app_version)?;
    backup::prune_backups(&backup_dir, 5)?;

    if renamed.exists() {
        fs::remove_file(&renamed)?;
    }
    fs::rename(&original, &renamed)?;

    fs::write(&original, hook_dll_bytes)?;

    let vpk = vpk_path(game_path);
    let item_count = if vpk.exists() {
        match item_db::generate_item_db(&vpk, &dir.join(ITEM_DB_NAME)) {
            Ok(count) => count,
            Err(e) => {
                let _ = uninstall(game_path);
                return Err(InstallerError::ItemDb(e.to_string()));
            }
        }
    } else {
        0
    };

    Ok(InstallResult {
        success: true,
        item_count,
        backup_id: backup_info.id,
    })
}

/// Uninstall the hook: remove proxy, restore original name
pub fn uninstall(game_path: &Path) -> Result<(), InstallerError> {
    let dir = dll_dir(game_path);
    let proxy = dir.join(ORIGINAL_DLL_NAME);
    let renamed = dir.join(RENAMED_ORIGINAL);
    let item_db = dir.join(ITEM_DB_NAME);

    if renamed.exists() {
        if proxy.exists() {
            fs::remove_file(&proxy)?;
        }
        fs::rename(&renamed, &proxy)?;
    }

    if item_db.exists() {
        fs::remove_file(&item_db)?;
    }

    Ok(())
}

/// Check if the hook is currently installed
pub fn status(game_path: &Path) -> HookStatus {
    let dir = dll_dir(game_path);
    let renamed = dir.join(RENAMED_ORIGINAL);
    let item_db = dir.join(ITEM_DB_NAME);

    if renamed.exists() {
        let item_count = if item_db.exists() {
            item_db::count_items_in_db(&item_db).unwrap_or(0)
        } else {
            0
        };
        HookStatus::Installed {
            version: env!("CARGO_PKG_VERSION").to_string(),
            item_count,
        }
    } else {
        HookStatus::NotInstalled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn setup_fake_game(dir: &Path) -> PathBuf {
        let game_path = dir.join("dota 2 beta");
        let dll_d = game_path.join("game").join("bin").join("win64");
        fs::create_dir_all(&dll_d).unwrap();

        let mut f = fs::File::create(dll_d.join(ORIGINAL_DLL_NAME)).unwrap();
        f.write_all(b"original steam api dll").unwrap();

        let vpk_d = game_path.join("game").join("dota");
        fs::create_dir_all(&vpk_d).unwrap();

        game_path
    }

    #[test]
    fn test_status_not_installed() {
        let dir = TempDir::new().unwrap();
        let game = setup_fake_game(dir.path());
        match status(&game) {
            HookStatus::NotInstalled => {}
            other => panic!("Expected NotInstalled, got {:?}", other),
        }
    }

    #[test]
    fn test_dll_dir() {
        let p = PathBuf::from("C:\\games\\dota 2 beta");
        assert_eq!(dll_dir(&p), p.join("game").join("bin").join("win64"));
    }
}
