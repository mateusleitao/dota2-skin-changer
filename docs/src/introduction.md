# Introduction

The Dota 2 Skin Changer is a Windows desktop application that unlocks the full Dota 2 cosmetic inventory locally by intercepting the Game Coordinator (GC) protocol.

## What It Does

When you install the hook and launch Dota 2:

1. The proxy DLL intercepts communication between the Dota 2 client and Valve's Game Coordinator
2. When inventory data arrives via the SOCache (Shared Object Cache), the proxy injects `CSOEconItem` protobuf entries for every cosmetic item in the game
3. The Dota 2 client's native **Heroes > Loadout** screen shows all cosmetics as available
4. You can equip any item through the game's standard UI

## What It Does NOT Do

- **Does not affect other players**: Cosmetics are client-side only. Other players see your actual equipped items.
- **Does not modify the Steam inventory**: Your Steam profile inventory remains unchanged.
- **Does not work on other games**: Only intercepts Dota 2's Game Coordinator messages.
- **Does not use memory injection**: Uses DLL proxy technique (file replacement), not runtime memory manipulation.

## Components

| Component | Language | Purpose |
|-----------|----------|---------|
| Launcher | Rust + React/TypeScript (Tauri v2) | Installer and management dashboard |
| dota2-hook | Rust (cdylib) | Proxy DLL that intercepts ISteamGameCoordinator |
| vpk-parser | Rust | Extracts item catalog from Dota 2's VPK files |
| dota2-proto | Rust | Compiled protobuf definitions for GC messages |
