# GC Hook Deep Dive

## Game Coordinator Protocol

Dota 2 manages inventory through the **Game Coordinator (GC)**, not through the standard Steam Inventory API (`ISteamInventory`). The GC communicates via Protocol Buffers over `ISteamGameCoordinator`.

### Interface

```
ISteamGameCoordinator (vtable):
  [0] SendMessage(msgType, data, size) → EGCResults
  [1] IsMessageAvailable(msgSize) → bool
  [2] RetrieveMessage(msgType, dest, destSize, msgSize) → EGCResults
```

### Message Flow

1. Client connects → sends `CMsgGCClientHello`
2. GC responds with `CMsgGCClientWelcome`
3. GC sends `CMsgSOCacheSubscribed` containing `CSOEconItem[]` — the player's inventory
4. Client renders items in the Loadout UI

## Hook Architecture

### DLL Proxy

The proxy DLL (`steam_api64.dll`) replaces the original, which is renamed to `steam_api64_o.dll`. All Steam API calls are forwarded to the original DLL unchanged, except for `ISteamGameCoordinator`.

### ISteamGameCoordinator Wrapper

When the game requests `ISteamGameCoordinator` via `GetISteamGenericInterface("SteamGameCoordinator001")`, the proxy returns a wrapper object that:

- **`SendMessage`**: Forwards to original (outgoing messages are not modified)
- **`IsMessageAvailable`**: Forwards to original
- **`RetrieveMessage`**: Calls original, then modifies the response

### Inventory Injection

When `RetrieveMessage` returns a message with type `k_ESOMsg_CacheSubscribed` (24):

1. The protobuf payload is decoded as `CMsgSOCacheSubscribed`
2. The `objects` array is scanned for `type_id == 1` (CSOEconItem)
3. New `CSOEconItem` entries are generated from `item_db.bin`:
   - `id`: Unique high-range ID (`2^40 + def_index`)
   - `account_id`: Extracted from the original SOCache response
   - `def_index`: From the item catalog
   - `quantity`: 1
   - `quality`: From the item catalog
4. The modified protobuf is re-encoded and returned to the client

### CSOEconItem Structure

```protobuf
message CSOEconItem {
  optional uint64 id = 1;            // Unique item instance ID
  optional uint32 account_id = 2;    // Owner's Steam account ID
  optional uint32 inventory = 3;     // Inventory position
  optional uint32 def_index = 4;     // Item definition (from items_game.txt)
  optional uint32 quantity = 5;      // Stack count
  optional uint32 level = 6;         // Item level
  optional uint32 quality = 7;       // Quality tier
  optional uint32 flags = 8;         // Bitfield flags
  optional uint32 origin = 9;        // How the item was obtained
  optional uint32 style = 15;        // Visual style variant
  optional uint64 original_id = 16;  // Original item ID
  repeated CSOEconItemEquipped equipped_state = 18;
}
```
