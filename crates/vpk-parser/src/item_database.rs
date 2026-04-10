use serde::{Deserialize, Serialize};

/// The complete item database, serialized to item_db.bin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDatabase {
    pub version: u32,
    pub items: Vec<ItemDefinition>,
}

/// A single cosmetic item definition extracted from items_game.txt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDefinition {
    pub def_index: u32,
    pub name: String,
    pub item_slot: String,
    pub hero_name: String,
    pub rarity: String,
    pub quality: u32,
}
