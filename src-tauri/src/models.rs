use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub path: String,
    pub vpk_path: String,
    pub steam_api_path: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HookStatus {
    NotInstalled,
    Installed { version: String, item_count: u32 },
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    pub success: bool,
    pub item_count: u32,
    pub backup_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub id: String,
    pub timestamp: String,
    pub sha256: String,
    pub app_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCatalog {
    pub total_items: u32,
    pub heroes: Vec<HeroCosmeticSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroCosmeticSummary {
    pub hero_name: String,
    pub item_count: u32,
    pub slots: Vec<String>,
}
