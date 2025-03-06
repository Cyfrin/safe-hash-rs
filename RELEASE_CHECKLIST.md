# Release checklist

This checklist is meant to be used as a guide for the release process.

## Requirements

- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [cargo-release](https://github.com/crate-ci/cargo-release)

## Steps

- [ ] Switch to `main` branch
- [ ] Run `cargo release patch --no-publish`. It will perform a dry run and make sure the changes are as intended. If satisfied, run `cargo release patch --no-publish execute`
