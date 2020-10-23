# Change Log

[UNRELEASED]

[0.7.5]

- Updated dependencies (including Seed 0.8.0).

[0.7.4]

- Change dist output location
- Temporarily switch test to Chrome - [Gecko fails](https://github.com/rustwasm/wasm-bindgen/issues/2261)

[0.7.3]

- Upgrade postcss-typed-css-classes to bring in bug fix

[0.7.2]

- Upgrade postcss-typed-css-classes

[0.7.1]

- dependabot > [Changelog](https://github.com/seed-rs/seed-quickstart-webpack/pull/21#issue-429231397)

[0.7.0]

- Moved files and directories located in `crate` directory to project root directory.
- Moved files and directories located in `configs` directory to project root directory.
- Moved files and directories located in `entries` directory to static directory.
- Moved `entries/index.ts` to project root directoy.
- Update/remove references to `crate`, `configs` and `entries` directories in configuration files and documentation.

[0.6.0]

- Updated all JS dependencies.
- Added `<base href="/">` and related comments to support non-root deploys.
- Updated all JS dependencies.
- App adapted to Seed 0.7.0.
- Disable unnecessary error message on the overlay.
- App refactored.
- Added `FUNDING.yml`.
- Updated `README.md`.

[0.5.0]

- `Seed` updated to `0.6.0`.
- [BREAKING] `wasm-pack` > `0.9.0` required.
- Deleted unnecessary `optimize-wasm.js` script.
- Minor `lib.rs` changes - removed `Clone` constraint on `Msg` and `prerendered` variable is replaced with a `Model`'s property.
- Added `wasm_bindgen_test_configure!(run_in_browser);` into `test.rs`.
- Disabled performance hints in `webpack.config.js`.
- Removed `critters-webpack-plugin`.

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
