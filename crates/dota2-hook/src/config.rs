use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::RwLock;
use vpk_parser::item_database::ItemDatabase;

static ITEM_DB: Lazy<RwLock<Option<ItemDatabase>>> = Lazy::new(|| RwLock::new(None));

/// Load the item database from item_db.bin next to the DLL
pub fn load_item_database() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_db_path();
    if !db_path.exists() {
        return Err(format!("item_db.bin not found at {}", db_path.display()).into());
    }

    let bytes = std::fs::read(&db_path)?;
    let db: ItemDatabase = bincode::deserialize(&bytes)?;

    let mut lock = ITEM_DB.write().map_err(|e| format!("Lock poisoned: {e}"))?;
    *lock = Some(db);

    Ok(())
}

/// Get a reference to the loaded item database
pub fn get_item_database() -> Option<ItemDatabase> {
    ITEM_DB.read().ok().and_then(|lock| lock.clone())
}

/// Get the number of items in the loaded database
pub fn item_count() -> u32 {
    get_item_database()
        .map(|db| db.items.len() as u32)
        .unwrap_or(0)
}

fn get_db_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let mut path = std::env::current_exe().unwrap_or_default();
        path.pop();
        path.push("item_db.bin");
        path
    }
    #[cfg(not(target_os = "windows"))]
    {
        PathBuf::from("item_db.bin")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vpk_parser::item_database::ItemDefinition;

    #[test]
    fn test_item_count_empty() {
        assert_eq!(item_count(), 0);
    }

    #[test]
    fn test_deserialize_item_db() {
        let db = ItemDatabase {
            version: 1,
            items: vec![ItemDefinition {
                def_index: 100,
                name: "Test".to_string(),
                item_slot: "weapon".to_string(),
                hero_name: "hero".to_string(),
                rarity: "common".to_string(),
                quality: 4,
            }],
        };

        let encoded = bincode::serialize(&db).unwrap();
        let decoded: ItemDatabase = bincode::deserialize(&encoded).unwrap();
        assert_eq!(decoded.items.len(), 1);
        assert_eq!(decoded.items[0].def_index, 100);
    }
}
