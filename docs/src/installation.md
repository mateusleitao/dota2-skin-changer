# Installation

## Requirements

- Windows 10 or later (64-bit)
- Dota 2 installed via Steam
- ~50 MB free disk space

## Installing

1. Download the latest release from [GitHub Releases](../../releases)
2. Run the `.msi` installer
3. Launch **Dota 2 Skin Changer** from the Start Menu
4. The application will auto-detect your Dota 2 installation
5. Click **Install Hook**
6. Launch Dota 2 normally

## Uninstalling

### Method 1: Via the Launcher

1. Open the Dota 2 Skin Changer launcher
2. Click **Uninstall**
3. The original `steam_api64.dll` is restored from backup

### Method 2: Via Steam

1. Open Steam
2. Right-click Dota 2 → Properties → Installed Files
3. Click **Verify integrity of game files**
4. Steam will restore any modified files

### Method 3: Manual

1. Navigate to `<Dota 2>/game/bin/win64/`
2. Delete `steam_api64.dll` (the proxy)
3. Rename `steam_api64_o.dll` back to `steam_api64.dll`
4. Delete `item_db.bin`
