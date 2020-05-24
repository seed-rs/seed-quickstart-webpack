# Release Checklist

1. Update `CHANGELOG.md` (content + increment version).
1. Update version also in `package.json`.
1. Run `cargo make verify`
1. Commit "v#.#.#".
1. Push.
1. Wait for the CI to go green.
1. Create GitHub release (create a new tag).
