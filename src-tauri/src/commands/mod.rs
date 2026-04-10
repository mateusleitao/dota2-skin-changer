use crate::game_path::{GamePathResolver, WindowsGamePathResolver};
use crate::models::*;
use crate::{backup, installer};
use std::path::PathBuf;

fn get_game_path() -> Result<PathBuf, String> {
    let resolver = WindowsGamePathResolver;
    resolver
        .find_dota2_path()
        .map_err(|e| format!("Failed to detect Dota 2: {e}"))
}

#[tauri::command]
pub fn detect_game() -> Result<GameInfo, String> {
    let path = get_game_path()?;
    let vpk = installer::vpk_path(&path);
    let steam_api = installer::dll_dir(&path).join("steam_api64.dll");

    Ok(GameInfo {
        path: path.display().to_string(),
        vpk_path: vpk.display().to_string(),
        steam_api_path: steam_api.display().to_string(),
        version: None,
    })
}

#[tauri::command]
pub fn get_hook_status() -> Result<HookStatus, String> {
    let path = get_game_path()?;
    Ok(installer::status(&path))
}

#[tauri::command]
pub fn install_hook() -> Result<InstallResult, String> {
    let path = get_game_path()?;

    // In production, this would be the embedded hook DLL bytes
    // For now, return an error indicating the DLL needs to be built
    let hook_dll_bytes = include_bytes_or_stub();

    installer::install(&path, &hook_dll_bytes, env!("CARGO_PKG_VERSION"))
        .map_err(|e| format!("Installation failed: {e}"))
}

#[tauri::command]
pub fn uninstall_hook() -> Result<(), String> {
    let path = get_game_path()?;
    installer::uninstall(&path).map_err(|e| format!("Uninstall failed: {e}"))
}

#[tauri::command]
pub fn get_backups() -> Result<Vec<BackupInfo>, String> {
    let path = get_game_path()?;
    let backup_dir = backup::default_backup_dir(&path);
    backup::list_backups(&backup_dir).map_err(|e| format!("Failed to list backups: {e}"))
}

#[tauri::command]
pub fn restore_backup(id: String) -> Result<(), String> {
    let path = get_game_path()?;
    let backup_dir = backup::default_backup_dir(&path);
    let dest = installer::dll_dir(&path).join("steam_api64.dll");

    let _ = installer::uninstall(&path);

    backup::restore_backup(&backup_dir, &id, &dest).map_err(|e| format!("Restore failed: {e}"))
}

#[tauri::command]
pub fn get_item_catalog() -> Result<ItemCatalog, String> {
    let path = get_game_path()?;
    let item_db_path = installer::dll_dir(&path).join("item_db.bin");

    if !item_db_path.exists() {
        return Ok(ItemCatalog {
            total_items: 0,
            heroes: vec![],
        });
    }

    let db = crate::item_db::load_item_db(&item_db_path)
        .map_err(|e| format!("Failed to load item database: {e}"))?;

    let mut hero_map: std::collections::HashMap<String, (u32, std::collections::HashSet<String>)> =
        std::collections::HashMap::new();

    for item in &db.items {
        let entry = hero_map
            .entry(item.hero_name.clone())
            .or_insert_with(|| (0, std::collections::HashSet::new()));
        entry.0 += 1;
        entry.1.insert(item.item_slot.clone());
    }

    let heroes: Vec<HeroCosmeticSummary> = hero_map
        .into_iter()
        .map(|(name, (count, slots))| HeroCosmeticSummary {
            hero_name: name,
            item_count: count,
            slots: slots.into_iter().collect(),
        })
        .collect();

    Ok(ItemCatalog {
        total_items: db.items.len() as u32,
        heroes,
    })
}

/// Stub: in production, this would be the compiled dota2-hook DLL
/// embedded via include_bytes!("path/to/dota2_hook.dll")
fn include_bytes_or_stub() -> Vec<u8> {
    Vec::new()
}
