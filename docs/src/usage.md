# Usage

## First-Time Setup

1. Open the Dota 2 Skin Changer launcher
2. Wait for automatic Dota 2 detection
3. Click **Install Hook** — this will:
   - Create a backup of the original `steam_api64.dll`
   - Install the proxy DLL
   - Generate the item database from your Dota 2 VPK files

## Using Cosmetics In-Game

1. Launch Dota 2 (via Steam, the launcher, or any other method)
2. Go to **Heroes** → select any hero → **Loadout**
3. All cosmetic items now appear as available
4. Equip items as you normally would
5. Play matches with your selected cosmetics

## Important Notes

- Cosmetic changes are **client-side only** — other players see your actual items
- The hook survives Dota 2 restarts — no need to reinstall each time
- Dota 2 game updates may require reinstalling the hook (the launcher will detect this)

## Updating the Item Database

When Dota 2 receives a major update with new cosmetics:

1. Open the launcher
2. Click **Refresh** in the Item Database section
3. This re-extracts `items_game.txt` from the VPK and regenerates `item_db.bin`
