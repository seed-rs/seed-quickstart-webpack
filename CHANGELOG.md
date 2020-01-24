# Change Log

[UNRELEASED]

[0.4.1]

- Ignore `clippy` lint rule `must_use_candidate`.
- Optimized build (WASM file size changed from 584 to 349 KB).
- `Seed` updated to `0.5.1` + updated `crate/src/lib.rs`.
- Updated all Rust dependencies.
- Updated all JS dependencies.

[0.4.0]

- `Seed` updated to `0.5.0` + refactored `crate/src/lib.rs`.
- All Rust and JS dependencies updated.
- GHA workflow `main.yml` - `on: [push]` changed to `on: [push, pull_request]`.

[0.3.1]

- `Travis CI` replaced with `Github Actions`.
- `Seed` updated to `0.4.2` + refactored `crate/src/lib.rs`.

[0.3.0]

- Quickstart is now based on `https://github.com/MartinKavik/kavik.cz`.
- Rewritten `README.md`.
- Previous version moved into branch `older`.

[0.2.0]

- Updated all dependencies + project adapted to them:
  - Set `outDir` in `WasmPackPlugin`.
  - Removed `css/tailwind.js`.
  - Changed paths in `entries/index.ts`.
  - Removed comments from `css/styles.css`.
  - Updated CSS classes in `crate/src/app.rs`.
  - Updated import of `ClearnWepackPlugin`.
- Dev server port changed from `3000` to `8000`.
- Added `netlify.toml` for better SPA support.
- `crate/*` changes:
  - Updated dependencies in `Cargo.toml` and `Seed` set to `master` (probably temporary).
  - Refactored `app.rs`, `lib.rs`, `rust_api.rs`, `clock.ts`, `ts_apis`, `seed_helpers.ts`.
  - Removed `seed_helpers.rs`.

[0.1.0]

- Initial version.
