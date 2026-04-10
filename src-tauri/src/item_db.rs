use std::fs;
use std::path::Path;
use thiserror::Error;
use vpk_parser::item_database::ItemDatabase;

#[derive(Error, Debug)]
pub enum ItemDbError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("VPK parse error: {0}")]
    VpkParse(String),
    #[error("KeyValues parse error: {0}")]
    KeyValuesParse(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Generate item_db.bin from the Dota 2 VPK file
pub fn generate_item_db(vpk_path: &Path, output_path: &Path) -> Result<u32, ItemDbError> {
    let items_text = vpk_parser::vpk::extract_items_game(vpk_path)
        .map_err(|e| ItemDbError::VpkParse(e.to_string()))?;

    let items = vpk_parser::keyvalues::parse_items_game(&items_text)
        .map_err(|e| ItemDbError::KeyValuesParse(e.to_string()))?;

    let db = ItemDatabase {
        version: 1,
        items: items.clone(),
    };

    let encoded =
        bincode::serialize(&db).map_err(|e| ItemDbError::Serialization(e.to_string()))?;
    fs::write(output_path, encoded)?;

    Ok(items.len() as u32)
}

/// Count items in an existing item_db.bin file
pub fn count_items_in_db(path: &Path) -> Result<u32, ItemDbError> {
    let bytes = fs::read(path)?;
    let db: ItemDatabase =
        bincode::deserialize(&bytes).map_err(|e| ItemDbError::Serialization(e.to_string()))?;
    Ok(db.items.len() as u32)
}

/// Load the full item database from a binary file
pub fn load_item_db(path: &Path) -> Result<ItemDatabase, ItemDbError> {
    let bytes = fs::read(path)?;
    let db: ItemDatabase =
        bincode::deserialize(&bytes).map_err(|e| ItemDbError::Serialization(e.to_string()))?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use vpk_parser::item_database::ItemDefinition;

    #[test]
    fn test_roundtrip_item_db() {
        let db = ItemDatabase {
            version: 1,
            items: vec![
                ItemDefinition {
                    def_index: 4000,
                    name: "Test Sword".to_string(),
                    item_slot: "weapon".to_string(),
                    hero_name: "npc_dota_hero_juggernaut".to_string(),
                    rarity: "rare".to_string(),
                    quality: 4,
                },
                ItemDefinition {
                    def_index: 4001,
                    name: "Test Helm".to_string(),
                    item_slot: "head".to_string(),
                    hero_name: "npc_dota_hero_juggernaut".to_string(),
                    rarity: "common".to_string(),
                    quality: 4,
                },
            ],
        };

        let dir = TempDir::new().unwrap();
        let path = dir.path().join("item_db.bin");

        let encoded = bincode::serialize(&db).unwrap();
        fs::write(&path, encoded).unwrap();

        let loaded = load_item_db(&path).unwrap();
        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.items.len(), 2);
        assert_eq!(loaded.items[0].def_index, 4000);
        assert_eq!(loaded.items[1].name, "Test Helm");
    }

    #[test]
    fn test_count_items() {
        let db = ItemDatabase {
            version: 1,
            items: vec![ItemDefinition {
                def_index: 100,
                name: "Item".to_string(),
                item_slot: "weapon".to_string(),
                hero_name: "hero".to_string(),
                rarity: "common".to_string(),
                quality: 4,
            }],
        };

        let dir = TempDir::new().unwrap();
        let path = dir.path().join("item_db.bin");
        let encoded = bincode::serialize(&db).unwrap();
        fs::write(&path, encoded).unwrap();

        assert_eq!(count_items_in_db(&path).unwrap(), 1);
    }
}
