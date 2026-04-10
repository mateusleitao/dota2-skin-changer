// Dota 2 Game Coordinator protobuf message definitions.
// Generated from the official protobuf definitions at
// https://github.com/SteamDatabase/GameTracking-Dota2/tree/master/Protobufs

include!(concat!(env!("OUT_DIR"), "/_.rs"));

/// SOCache type IDs used by the Dota 2 Game Coordinator
pub mod so_type {
    /// CSOEconItem — individual inventory items
    pub const ECON_ITEM: i32 = 1;
    /// CSOEconGameAccountClient — account-level econ data
    pub const GAME_ACCOUNT_CLIENT: i32 = 7;
}

/// GC message type IDs
pub mod gc_msg {
    /// k_EMsgGCClientWelcome
    pub const CLIENT_WELCOME: u32 = 4004;
    /// k_EMsgGCClientHello
    pub const CLIENT_HELLO: u32 = 4006;
    /// k_ESOMsg_CacheSubscribed
    pub const SO_CACHE_SUBSCRIBED: u32 = 24;
    /// k_ESOMsg_Create
    pub const SO_CREATE: u32 = 21;
    /// k_ESOMsg_Update
    pub const SO_UPDATE: u32 = 22;
    /// k_ESOMsg_Destroy
    pub const SO_DESTROY: u32 = 23;
    /// k_ESOMsg_UpdateMultiple
    pub const SO_UPDATE_MULTIPLE: u32 = 26;
}

/// Protobuf header size: 4 bytes msg_type + 4 bytes body length
pub const GC_MSG_HEADER_SIZE: usize = 8;

/// Mask to extract actual message type (strip protobuf flag bit)
pub const GC_MSG_TYPE_MASK: u32 = 0x7FFFFFFF;
/// Flag indicating the message uses protobuf encoding
pub const GC_MSG_PROTO_FLAG: u32 = 0x80000000;

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;

    #[test]
    fn test_cso_econ_item_roundtrip() {
        let item = CsoEconItem {
            id: Some(999_000_000_001),
            account_id: Some(12345678),
            inventory: Some(1),
            def_index: Some(4000),
            quantity: Some(1),
            level: Some(1),
            quality: Some(4),
            flags: Some(0),
            origin: Some(0),
            style: Some(0),
            original_id: Some(999_000_000_001),
            attribute: vec![],
            interior_item: None,
            equipped_state: vec![],
        };

        let mut buf = Vec::new();
        item.encode(&mut buf).unwrap();

        let decoded = CsoEconItem::decode(&buf[..]).unwrap();
        assert_eq!(decoded.id, Some(999_000_000_001));
        assert_eq!(decoded.def_index, Some(4000));
        assert_eq!(decoded.account_id, Some(12345678));
        assert_eq!(decoded.quantity, Some(1));
    }

    #[test]
    fn test_cso_econ_item_equipped_state() {
        let item = CsoEconItem {
            id: Some(1),
            account_id: Some(1),
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
            equipped_state: vec![CsoEconItemEquipped {
                new_class: Some(1),
                new_slot: Some(0),
            }],
        };

        let mut buf = Vec::new();
        item.encode(&mut buf).unwrap();
        let decoded = CsoEconItem::decode(&buf[..]).unwrap();

        assert_eq!(decoded.equipped_state.len(), 1);
        assert_eq!(decoded.equipped_state[0].new_class, Some(1));
        assert_eq!(decoded.equipped_state[0].new_slot, Some(0));
    }

    #[test]
    fn test_so_cache_subscribed_roundtrip() {
        let item = CsoEconItem {
            id: Some(1),
            def_index: Some(4000),
            account_id: Some(123),
            inventory: Some(1),
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

        let mut item_bytes = Vec::new();
        item.encode(&mut item_bytes).unwrap();

        let cache = CMsgSoCacheSubscribed {
            objects: vec![c_msg_so_cache_subscribed::SubscribedType {
                type_id: Some(so_type::ECON_ITEM),
                object_data: vec![item_bytes],
            }],
            version: Some(1),
            owner_soid: Some(CMsgSoidOwner {
                r#type: Some(1),
                id: Some(123),
            }),
            service_id: None,
            service_list: vec![],
            sync_version: None,
        };

        let mut buf = Vec::new();
        cache.encode(&mut buf).unwrap();

        let decoded = CMsgSoCacheSubscribed::decode(&buf[..]).unwrap();
        assert_eq!(decoded.objects.len(), 1);
        assert_eq!(decoded.objects[0].type_id, Some(so_type::ECON_ITEM));
        assert_eq!(decoded.objects[0].object_data.len(), 1);

        let decoded_item =
            CsoEconItem::decode(decoded.objects[0].object_data[0].as_slice()).unwrap();
        assert_eq!(decoded_item.def_index, Some(4000));
    }
}
