# Change Log

This project adheres to [Semantic Versioning](http://semver.org/).

[UNRELEASED]

[0.2.0]

- Updated all dependencies + project adapted to them:
  - Set `outDir` in `WasmPackPlugin`
  - Removed `css/tailwind.js`
  - Changed paths in `entries/index.ts`
  - Removed comments from `css/styles.css`
  - Updated CSS classes in `crate/src/app.rs`
  - Updated import of `ClearnWepackPlugin`
- Dev server port changed from `3000` to `8000`
- Added `netlify.toml` for better SPA support
- `crate/*` changes:
  - Updated dependencies in `Cargo.toml` and `Seed` set to `master` (probably temporary)
  - Refactored `app.rs`, `lib.rs`, `rust_api.rs`, `clock.ts`, `ts_apis`, `seed_helpers.ts`
  - Removed `seed_helpers.rs`

[0.1.0]

- Initial version
