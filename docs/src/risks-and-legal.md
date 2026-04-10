# Risks and Legal

## VAC (Valve Anti-Cheat)

**This software replaces `steam_api64.dll` in the Dota 2 installation directory.** This is a file that VAC monitors. Using this tool carries a **real risk of VAC ban**.

### Risk Mitigation

- Use on an alternate/secondary account
- The hook can be uninstalled instantly via the launcher or Steam's "Verify integrity of game files"
- The proxy DLL does not modify the game's executable or inject code into a running process

### What VAC Checks

VAC may detect:

- Modified or replaced game DLLs
- DLL signatures that don't match Valve's expected hashes
- Unusual behavior in Steam API calls

## Terms of Service

Using this software violates Valve's Terms of Service and Dota 2's Subscriber Agreement. Specifically:

- Modification of game files
- Circumventing item ownership verification
- Using third-party tools that interact with the game client

## Disclaimer

This software is provided for **educational and research purposes only**.

- The authors assume no responsibility for any consequences of using this software
- Users accept all risk including potential account bans
- This project is not affiliated with, endorsed by, or connected to Valve Corporation
- Cosmetic items remain the property of Valve Corporation

## Client-Side Nature

All modifications are strictly client-side:

- Other players cannot see your cosmetic changes
- Server-side item verification remains intact
- No items are actually created, traded, or transferred
- Your Steam inventory is not modified
