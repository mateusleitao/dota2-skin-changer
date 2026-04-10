use crate::models::BackupInfo;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Backup not found: {0}")]
    NotFound(String),
    #[error("SHA256 mismatch: expected {expected}, got {actual}")]
    IntegrityError { expected: String, actual: String },
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct BackupMetadata {
    id: String,
    timestamp: String,
    sha256: String,
    app_version: String,
    original_filename: String,
}

/// Compute SHA256 hash of a file
pub fn sha256_file(path: &Path) -> Result<String, BackupError> {
    let bytes = fs::read(path)?;
    let hash = Sha256::digest(&bytes);
    Ok(hex::encode(hash))
}

/// Create a backup of the original steam_api64.dll
pub fn create_backup(
    source: &Path,
    backup_dir: &Path,
    app_version: &str,
) -> Result<BackupInfo, BackupError> {
    let id = uuid::Uuid::new_v4().to_string();
    let timestamp = chrono::Utc::now().to_rfc3339();
    let sha256 = sha256_file(source)?;

    let backup_path = backup_dir.join(&id);
    fs::create_dir_all(&backup_path)?;

    let dest = backup_path.join("steam_api64.dll");
    fs::copy(source, &dest)?;

    let verify_hash = sha256_file(&dest)?;
    if verify_hash != sha256 {
        return Err(BackupError::IntegrityError {
            expected: sha256,
            actual: verify_hash,
        });
    }

    let metadata = BackupMetadata {
        id: id.clone(),
        timestamp: timestamp.clone(),
        sha256: sha256.clone(),
        app_version: app_version.to_string(),
        original_filename: "steam_api64.dll".to_string(),
    };

    let meta_json = serde_json::to_string_pretty(&metadata)
        .map_err(|e| BackupError::Serialization(e.to_string()))?;
    fs::write(backup_path.join("metadata.json"), meta_json)?;

    Ok(BackupInfo {
        id,
        timestamp,
        sha256,
        app_version: app_version.to_string(),
    })
}

/// Restore a backup by ID
pub fn restore_backup(backup_dir: &Path, backup_id: &str, dest: &Path) -> Result<(), BackupError> {
    let backup_path = backup_dir.join(backup_id);
    if !backup_path.exists() {
        return Err(BackupError::NotFound(backup_id.to_string()));
    }

    let meta_str = fs::read_to_string(backup_path.join("metadata.json"))?;
    let metadata: BackupMetadata =
        serde_json::from_str(&meta_str).map_err(|e| BackupError::Serialization(e.to_string()))?;

    let source = backup_path.join(&metadata.original_filename);
    let source_hash = sha256_file(&source)?;
    if source_hash != metadata.sha256 {
        return Err(BackupError::IntegrityError {
            expected: metadata.sha256,
            actual: source_hash,
        });
    }

    fs::copy(&source, dest)?;
    Ok(())
}

/// List all available backups
pub fn list_backups(backup_dir: &Path) -> Result<Vec<BackupInfo>, BackupError> {
    let mut backups = Vec::new();

    if !backup_dir.exists() {
        return Ok(backups);
    }

    for entry in fs::read_dir(backup_dir)? {
        let entry = entry?;
        let meta_path = entry.path().join("metadata.json");
        if meta_path.exists() {
            let meta_str = fs::read_to_string(&meta_path)?;
            if let Ok(metadata) = serde_json::from_str::<BackupMetadata>(&meta_str) {
                backups.push(BackupInfo {
                    id: metadata.id,
                    timestamp: metadata.timestamp,
                    sha256: metadata.sha256,
                    app_version: metadata.app_version,
                });
            }
        }
    }

    backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(backups)
}

/// Prune old backups, keeping at most `max_backups`
pub fn prune_backups(backup_dir: &Path, max_backups: usize) -> Result<(), BackupError> {
    let mut backups = list_backups(backup_dir)?;

    if backups.len() <= max_backups {
        return Ok(());
    }

    backups.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    for old in backups.iter().skip(max_backups) {
        let old_path = backup_dir.join(&old.id);
        if old_path.exists() {
            fs::remove_dir_all(&old_path)?;
        }
    }

    Ok(())
}

/// Get the default backup directory relative to a game path
pub fn default_backup_dir(game_path: &Path) -> PathBuf {
    game_path
        .parent()
        .unwrap_or(game_path)
        .join("dota2_skin_changer_backups")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(content).unwrap();
        path
    }

    #[test]
    fn test_sha256_file() {
        let dir = TempDir::new().unwrap();
        let file = create_test_file(dir.path(), "test.bin", b"hello world");
        let hash = sha256_file(&file).unwrap();
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_create_and_restore_backup() {
        let dir = TempDir::new().unwrap();
        let source = create_test_file(dir.path(), "steam_api64.dll", b"fake dll content");
        let backup_dir = dir.path().join("backups");

        let info = create_backup(&source, &backup_dir, "0.1.0").unwrap();
        assert!(!info.id.is_empty());
        assert!(!info.sha256.is_empty());

        let restore_dest = dir.path().join("restored.dll");
        restore_backup(&backup_dir, &info.id, &restore_dest).unwrap();

        assert_eq!(fs::read(&restore_dest).unwrap(), b"fake dll content");
    }

    #[test]
    fn test_list_backups_empty() {
        let dir = TempDir::new().unwrap();
        let backups = list_backups(&dir.path().join("nonexistent")).unwrap();
        assert!(backups.is_empty());
    }

    #[test]
    fn test_prune_backups() {
        let dir = TempDir::new().unwrap();
        let source = create_test_file(dir.path(), "steam_api64.dll", b"dll");
        let backup_dir = dir.path().join("backups");

        for _ in 0..5 {
            create_backup(&source, &backup_dir, "0.1.0").unwrap();
        }

        let before = list_backups(&backup_dir).unwrap();
        assert_eq!(before.len(), 5);

        prune_backups(&backup_dir, 2).unwrap();

        let after = list_backups(&backup_dir).unwrap();
        assert_eq!(after.len(), 2);
    }
}
