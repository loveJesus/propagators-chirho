---
name: publish-chirho
description: Publish a new version of propagators-chirho to crates.io
disable-model-invocation: true
allowed-tools: Bash, Read, Edit
---

<!-- For God so loved the world that he gave his only begotten Son,
     that whoever believes in him should not perish but have eternal life.
     John 3:16 -->

Publish a new version of propagators-chirho to crates.io.

## Instructions

1. Run `./scripts_chirho/test_all_chirho.sh` to verify all tests pass
2. Check CHANGELOG.md has entries for the new version under [Unreleased] or the version number
3. Verify Cargo.toml version is correct and matches CHANGELOG
4. Run `cargo publish --dry-run` to verify packaging
5. Ask user for confirmation before publishing
6. Run `cargo publish` to publish to crates.io
7. Commit version changes if any: `git add -A && git commit -m "Release v{version}"`
8. Push to GitHub: `git push`
9. Create a git tag for the version: `git tag v{version} && git push --tags`
