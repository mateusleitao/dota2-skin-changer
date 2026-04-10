# Architecture

## System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Launcher (Tauri v2)                       │
│  ┌─────────────┐  ┌──────────┐  ┌────────────────────────┐  │
│  │ Game Detect  │  │ Installer│  │ VPK Parser + Item DB   │  │
│  │ (Registry)   │  │ (Backup) │  │ (items_game.txt parse) │  │
│  └─────────────┘  └──────────┘  └────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                           │
                    Installs files
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│              Dota 2 Installation (Windows)                   │
│  game/bin/win64/                                            │
│  ├── steam_api64.dll      ← Proxy DLL (dota2-hook)         │
│  ├── steam_api64_o.dll    ← Original (renamed)             │
│  └── item_db.bin          ← Serialized item catalog         │
└─────────────────────────────────────────────────────────────┘
```

## Crate Dependency Graph

```
dota2-skin-changer (Tauri app)
├── vpk-parser
│   └── (standalone)
├── dota2-proto
│   └── prost (protobuf)
└── dota2-hook (cdylib)
    ├── dota2-proto
    ├── vpk-parser
    └── windows-rs
```

## Data Flow

1. **Launcher** reads Dota 2's `pak01_dir.vpk`
2. **vpk-parser** extracts `scripts/items/items_game.txt`
3. **KeyValues parser** extracts all item definitions (defindex, hero, slot, rarity)
4. Serialized to `item_db.bin` via bincode
5. **dota2-hook** loads `item_db.bin` at DLL attach time
6. When the Game Coordinator sends `CMsgSOCacheSubscribed`, the hook injects `CSOEconItem` entries for every item in the database
7. The Dota 2 client displays all items as owned
