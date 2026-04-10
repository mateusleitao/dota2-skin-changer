# Development

## Prerequisites

- Rust 1.75+ (`rustup install stable`)
- Node.js 20+
- Protocol Buffers compiler (`protoc`)

### macOS

```bash
brew install protobuf node
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

```powershell
winget install Rustlang.Rustup
winget install OpenJS.NodeJS
winget install Google.Protobuf
```

## Building

```bash
# Install frontend dependencies
npm install

# Build all Rust crates (library mode — works on macOS)
cargo build --workspace --lib

# Run all tests
cargo test --workspace
npm test

# Development server (frontend only)
npm run dev
```

## Testing on macOS

All tests are designed to run on macOS via fixtures. No Windows machine or Dota 2 installation is required.

### Rust Tests

- **dota2-proto**: Protobuf round-trip serialization tests
- **vpk-parser**: VPK header parsing, KeyValues/VDF parsing, item extraction
- **dota2-hook**: Inventory injection into SOCache, config loading, protobuf modification
- **src-tauri**: Backup cycle, game path detection (mock registry), installer sequence

### Frontend Tests

- **Dashboard**: Component rendering, status display
- **appStore**: Zustand state management

### Running Tests

```bash
# All Rust tests
cargo test --workspace

# Specific crate
cargo test -p dota2-hook

# Frontend tests
npm test

# Watch mode
npm run test:watch
```

## Project Structure

```
crates/
├── dota2-proto/    # Protobuf definitions (prost-build)
├── vpk-parser/     # VPK + KeyValues parser
└── dota2-hook/     # Proxy DLL (cdylib, Windows-only for DLL features)

src/                # React/TypeScript frontend
src-tauri/          # Tauri v2 backend
tests/fixtures/     # Test fixtures
docs/               # mdBook documentation
```
