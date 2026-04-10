# Architectural Decisions Record

This document tracks all major architectural decisions for the Dota 2 Skin Changer project.

## Decision 1: Windows-only Target

**Status**: Accepted  
**Date**: 2026-04-09

The application targets Windows exclusively. Windows Registry access, VPK file paths, and the DLL proxy mechanism are all Windows-specific. Attempting cross-platform support would add complexity with no real benefit since Dota 2's competitive player base is overwhelmingly on Windows.

## Decision 2: DLL Proxy + Game Coordinator Hook

**Status**: Accepted (replaces original VPK-only approach)  
**Date**: 2026-04-09

The Dota 2 client manages inventory through the **Game Coordinator (GC)**, not through the standard `ISteamInventory` interface. To make all cosmetics appear as owned in the native loadout UI, we intercept `ISteamGameCoordinator::RetrieveMessage` via a proxy DLL that replaces `steam_api64.dll`. The proxy forwards all other Steam API calls to the original DLL (renamed to `steam_api64_o.dll`) and only modifies GC messages containing `CMsgSOCacheSubscribed` to inject `CSOEconItem` entries for every cosmetic item.

**Trade-off**: Higher VAC detection risk than pure file modification, but this is the only technique that makes items appear in the native Dota 2 loadout screen.

## Decision 3: Tauri v2 (Rust + React + TypeScript)

**Status**: Accepted  
**Date**: 2026-04-09

The launcher uses Tauri v2 for its lightweight footprint, Rust-powered backend (sharing code with the VPK parser), and strong security model via capabilities. React + TypeScript provide a modern frontend with type safety.

## Decision 4: Test-Driven Development with Fixtures

**Status**: Accepted  
**Date**: 2026-04-09

All Rust modules have comprehensive tests using fixtures in `tests/fixtures/`. This enables primary development on macOS without requiring a Windows machine or Dota 2 installation. Fixtures include mock VPK files, sample `items_game.txt`, and serialized protobuf responses.

## Decision 5: Professional CI/CD Pipeline

**Status**: Accepted  
**Date**: 2026-04-09

GitHub Actions with matrix testing (ubuntu + windows), CodeQL scanning, Dependabot, conventional commits enforcement, and automated mdBook documentation deployment to GitHub Pages.

## Decision 6: Launcher as Installer/Dashboard

**Status**: Accepted (replaces original LoadoutBuilder UI)  
**Date**: 2026-04-09

Since cosmetic selection happens within Dota 2's native loadout UI (made possible by the GC hook), the launcher serves as an installer/manager rather than a loadout builder. It handles: game detection, hook installation/removal, backup management, and item database generation.

## Decision 7: Documentation as Code

**Status**: Accepted  
**Date**: 2026-04-09

All documentation lives in the repository as mdBook sources, deployed automatically to GitHub Pages. This file (DECISIONS.md) is the canonical record of architectural choices.

## Decision 8: Cursor Rules for Consistency

**Status**: Accepted  
**Date**: 2026-04-09

Custom Cursor rules enforce: mandatory tests for new Rust code, prohibition of techniques beyond DLL proxy + GC hook, documentation updates for significant changes, conventional commits, and consistent naming conventions.

## Decision 9: Protobuf Definitions from SteamDatabase

**Status**: Accepted  
**Date**: 2026-04-09

Protobuf definitions are sourced from the public [SteamDatabase/GameTracking-Dota2](https://github.com/SteamDatabase/GameTracking-Dota2/tree/master/Protobufs) repository. These are compiled with `prost-build` into the `dota2-proto` crate. Key messages: `CSOEconItem`, `CMsgSOCacheSubscribed`, `CSOEconItemEquipped`.
