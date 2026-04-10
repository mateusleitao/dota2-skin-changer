# Dota 2 Skin Changer

A production-grade Windows desktop application that unlocks the full Dota 2 cosmetic inventory locally by intercepting the Game Coordinator protocol.

## How It Works

The application installs a proxy DLL (`steam_api64.dll`) into your Dota 2 installation that intercepts communication between the game client and Valve's Game Coordinator. When the GC sends your inventory data via the SOCache (Shared Object Cache), the proxy injects `CSOEconItem` entries for every cosmetic item in the game — making them all appear as owned in the native Heroes > Loadout screen.

```
Dota 2 Client  →  Proxy DLL  →  Original steam_api64.dll  →  Steam Servers
                     ↓
              Intercepts GC messages
              Injects CSOEconItem[]
              into SOCache responses
```

## Important Disclaimers

> **VAC Risk**: This software modifies game files (`steam_api64.dll`). Using it **may result in a VAC ban**. Use at your own risk, preferably on an alternate account.

> **Terms of Service**: This tool violates Valve's Terms of Service for Dota 2. The authors assume no responsibility for any consequences of using this software.

> **Client-Side Only**: Cosmetic changes are only visible to you. Other players in your matches see your actual equipped items as authorized by the server.

## Architecture

| Component | Description |
|-----------|-------------|
| **Launcher** (Tauri v2) | Desktop app for installing/managing the hook |
| **dota2-hook** (cdylib) | Proxy DLL that intercepts ISteamGameCoordinator |
| **vpk-parser** | Extracts item definitions from Dota 2's VPK files |
| **dota2-proto** | Compiled protobuf definitions for GC messages |

## Installation

1. Download the latest release from [GitHub Releases](../../releases)
2. Run the installer (.msi)
3. Open the launcher — it will auto-detect your Dota 2 installation
4. Click **Install Hook**
5. Launch Dota 2 and open Heroes > Loadout — all cosmetics are available

## Uninstalling

1. Open the launcher and click **Uninstall Hook**, or
2. Use Steam's "Verify Integrity of Game Files" to restore originals

## Development

### Prerequisites

- Rust 1.75+ with `x86_64-pc-windows-msvc` target
- Node.js 20+
- Protobuf compiler (`protoc`)

### macOS Development (via fixtures)

All Rust tests use fixtures in `tests/fixtures/` — no Windows machine or Dota 2 installation required:

```bash
# Install dependencies
npm install

# Run Rust tests
cargo test --workspace

# Run frontend tests
npm test

# Development server (frontend only)
npm run dev
```

### Windows Development (full build)

```bash
# Full Tauri build
npm run tauri build

# Build only the hook DLL
cargo build -p dota2-hook --release
```

## License

This project is for educational and research purposes only.
