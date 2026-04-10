use dota2_proto::{
    c_msg_so_cache_subscribed, CMsgSoCacheSubscribed, CsoEconItem,
};
use prost::Message;
use vpk_parser::item_database::ItemDatabase;

/// Base ID for injected items — high range to avoid conflicts with real items
const INJECTED_ITEM_BASE_ID: u64 = 1 << 40;

/// SOCache type ID for CSOEconItem
const ECON_ITEM_TYPE: i32 = 1;

/// Generate CSOEconItem entries for all items in the database
pub fn generate_items(db: &ItemDatabase, account_id: u32) -> Vec<CsoEconItem> {
    db.items
        .iter()
        .enumerate()
        .map(|(idx, def)| {
            let item_id = INJECTED_ITEM_BASE_ID + def.def_index as u64;
            #[allow(clippy::cast_possible_truncation)]
            CsoEconItem {
                id: Some(item_id),
                account_id: Some(account_id),
                inventory: Some(idx as u32 + 1),
                def_index: Some(def.def_index),
                quantity: Some(1),
                level: Some(1),
                quality: Some(def.quality),
                flags: Some(0),
                origin: Some(0),
                style: Some(0),
                original_id: Some(item_id),
                attribute: vec![],
                interior_item: None,
                equipped_state: vec![],
            }
        })
        .collect()
}

/// Inject all items from the database into a CMsgSOCacheSubscribed message
pub fn inject_items_into_cache(
    cache_bytes: &[u8],
    db: &ItemDatabase,
    account_id: u32,
) -> Result<Vec<u8>, prost::DecodeError> {
    let mut cache = CMsgSoCacheSubscribed::decode(cache_bytes)?;

    let items = generate_items(db, account_id);

    let item_bytes: Vec<Vec<u8>> = items
        .iter()
        .map(|item| {
            let mut buf = Vec::new();
            item.encode(&mut buf).expect("Failed to encode CSOEconItem");
            buf
        })
        .collect();

    let existing_econ = cache
        .objects
        .iter_mut()
        .find(|obj| obj.type_id == Some(ECON_ITEM_TYPE));

    match existing_econ {
        Some(existing) => {
            existing.object_data.extend(item_bytes);
        }
        None => {
            cache
                .objects
                .push(c_msg_so_cache_subscribed::SubscribedType {
                    type_id: Some(ECON_ITEM_TYPE),
                    object_data: item_bytes,
                });
        }
    }

    let mut output = Vec::new();
    cache
        .encode(&mut output)
        .expect("Failed to encode modified cache");
    Ok(output)
}

/// Extract the account_id from a CMsgSOCacheSubscribed message
pub fn extract_account_id(cache_bytes: &[u8]) -> Option<u32> {
    let cache = CMsgSoCacheSubscribed::decode(cache_bytes).ok()?;
    #[allow(clippy::cast_possible_truncation)]
    cache.owner_soid.as_ref().and_then(|soid| soid.id.map(|id| id as u32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dota2_proto::CMsgSoidOwner;
    use vpk_parser::item_database::ItemDefinition;

    fn make_test_db() -> ItemDatabase {
        ItemDatabase {
            version: 1,
            items: vec![
                ItemDefinition {
                    def_index: 4000,
                    name: "Sword of Voth".to_string(),
                    item_slot: "weapon".to_string(),
                    hero_name: "npc_dota_hero_lc".to_string(),
                    rarity: "rare".to_string(),
                    quality: 4,
                },
                ItemDefinition {
                    def_index: 4001,
                    name: "Arcana Cape".to_string(),
                    item_slot: "back".to_string(),
                    hero_name: "npc_dota_hero_cm".to_string(),
                    rarity: "arcana".to_string(),
                    quality: 6,
                },
            ],
        }
    }

    #[test]
    fn test_generate_items() {
        let db = make_test_db();
        let items = generate_items(&db, 12345);

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].def_index, Some(4000));
        assert_eq!(items[0].account_id, Some(12345));
        assert!(items[0].id.unwrap() >= INJECTED_ITEM_BASE_ID);
        assert_eq!(items[1].def_index, Some(4001));
        assert_eq!(items[1].quality, Some(6));
    }

    #[test]
    fn test_inject_items_into_empty_cache() {
        let db = make_test_db();

        let cache = CMsgSoCacheSubscribed {
            objects: vec![],
            version: Some(1),
            owner_soid: Some(CMsgSoidOwner {
                r#type: Some(1),
                id: Some(12345),
            }),
            service_id: None,
            service_list: vec![],
            sync_version: None,
        };

        let mut cache_bytes = Vec::new();
        cache.encode(&mut cache_bytes).unwrap();

        let modified = inject_items_into_cache(&cache_bytes, &db, 12345).unwrap();

        let decoded = CMsgSoCacheSubscribed::decode(modified.as_slice()).unwrap();
        assert_eq!(decoded.objects.len(), 1);
        assert_eq!(decoded.objects[0].type_id, Some(ECON_ITEM_TYPE));
        assert_eq!(decoded.objects[0].object_data.len(), 2);

        let item0 = CsoEconItem::decode(decoded.objects[0].object_data[0].as_slice()).unwrap();
        assert_eq!(item0.def_index, Some(4000));
    }

    #[test]
    fn test_inject_items_into_existing_cache() {
        let db = make_test_db();

        let existing_item = CsoEconItem {
            id: Some(1),
            account_id: Some(12345),
            inventory: Some(1),
            def_index: Some(100),
            quantity: Some(1),
            level: Some(1),
            quality: Some(4),
            flags: Some(0),
            origin: Some(0),
            style: Some(0),
            original_id: Some(1),
            attribute: vec![],
            interior_item: None,
            equipped_state: vec![],
        };

        let mut existing_bytes = Vec::new();
        existing_item.encode(&mut existing_bytes).unwrap();

        let cache = CMsgSoCacheSubscribed {
            objects: vec![c_msg_so_cache_subscribed::SubscribedType {
                type_id: Some(ECON_ITEM_TYPE),
                object_data: vec![existing_bytes],
            }],
            version: Some(1),
            owner_soid: Some(CMsgSoidOwner {
                r#type: Some(1),
                id: Some(12345),
            }),
            service_id: None,
            service_list: vec![],
            sync_version: None,
        };

        let mut cache_bytes = Vec::new();
        cache.encode(&mut cache_bytes).unwrap();

        let modified = inject_items_into_cache(&cache_bytes, &db, 12345).unwrap();
        let decoded = CMsgSoCacheSubscribed::decode(modified.as_slice()).unwrap();

        // 1 existing + 2 injected = 3
        assert_eq!(decoded.objects[0].object_data.len(), 3);
    }

    #[test]
    fn test_extract_account_id() {
        let cache = CMsgSoCacheSubscribed {
            objects: vec![],
            version: Some(1),
            owner_soid: Some(CMsgSoidOwner {
                r#type: Some(1),
                id: Some(99887766),
            }),
            service_id: None,
            service_list: vec![],
            sync_version: None,
        };

        let mut buf = Vec::new();
        cache.encode(&mut buf).unwrap();

        assert_eq!(extract_account_id(&buf), Some(99887766));
    }
}
