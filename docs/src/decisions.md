# Architectural Decisions

This document mirrors `DECISIONS.md` in the repository root.

## Decision 1: Windows-only Target

The application targets Windows exclusively. Registry access, VPK paths, and the DLL proxy mechanism are Windows-specific.

## Decision 2: DLL Proxy + GC Hook

The Dota 2 client manages inventory through the Game Coordinator, not `ISteamInventory`. We intercept `ISteamGameCoordinator::RetrieveMessage` via a proxy DLL to inject items into the SOCache.

## Decision 3: Tauri v2

Lightweight launcher with Rust backend (shared code with VPK parser) and React/TypeScript frontend.

## Decision 4: Test-Driven with Fixtures

All modules have comprehensive tests using fixtures, enabling macOS development without Windows or Dota 2.

## Decision 5: Professional CI/CD

GitHub Actions with matrix testing, CodeQL, Dependabot, and automated documentation.

## Decision 6: Launcher as Installer/Dashboard

Cosmetic selection happens in Dota 2's native UI. The launcher only manages the hook installation.

## Decision 7: Documentation as Code

mdBook + DECISIONS.md, deployed automatically to GitHub Pages.

## Decision 8: Cursor Rules

Mandatory tests, no techniques beyond GC hook, documentation updates required.

## Decision 9: SteamDatabase Protobufs

Official public protobuf definitions compiled with prost-build.
