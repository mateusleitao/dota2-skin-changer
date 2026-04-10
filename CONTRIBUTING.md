# Contributing

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new feature
fix: fix a bug
docs: documentation changes
test: add or update tests
refactor: code refactoring without behavior change
chore: build, CI, or tooling changes
```

## Branch Naming

```
feat/short-description
fix/short-description
docs/short-description
```

## Pull Request Process

1. Create a feature branch from `main`
2. Ensure all tests pass (`cargo test --workspace && npm test`)
3. Update `DECISIONS.md` if making architectural changes
4. Run `cargo clippy -- -D warnings` and `cargo fmt --check`
5. Open a PR with a clear description of changes

## Development Guidelines

- Every new Rust module must have associated tests
- Use `thiserror` for error types, never `unwrap()` in production code
- Only GC message interception via DLL proxy is permitted — no memory scanning or direct process injection
- All fixtures go in `tests/fixtures/`
- Frontend state management uses Zustand
- Tailwind CSS for styling with the project's dark theme

## Testing on macOS

The test suite is designed to run entirely on macOS using fixtures. Windows-specific code uses trait-based abstractions with mock implementations for testing.
