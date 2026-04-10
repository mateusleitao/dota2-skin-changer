# Test Fixtures

This directory contains test fixtures for running the full test suite on macOS without a Windows machine or Dota 2 installation.

## Files

| File | Purpose |
|------|---------|
| `items_game_sample.txt` | Sample items_game.txt with ~10 items across 3 heroes, including prefab inheritance |
| `gameinfo.gi` | Sanitized copy of Dota 2's gameinfo.gi for path detection tests |

## Creating New Fixtures

When adding new test fixtures:

1. Place them in this directory
2. Document them in this README
3. Ensure they don't contain any real player data or Steam credentials
4. Keep fixtures minimal — only include data needed for the specific test
