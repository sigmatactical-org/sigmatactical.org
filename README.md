# sigmatactical.org

[![CI](https://github.com/sigmatactical-org/sigmatactical.org/actions/workflows/ci.yml/badge.svg)](https://github.com/sigmatactical-org/sigmatactical.org/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

Public open-source showcase for [sigmatactical.org](https://sigmatactical.org). Lists
repositories from the configured GitHub organization using the shared
[sigma-theme](https://github.com/sigmatactical-org/sigma-theme) chrome.

## Environment

| Variable | Purpose |
| --- | --- |
| `PORT` | Listen port (default `8080`) |
| `STORG_PUBLIC_BASE_URL` | This site's public URL |
| `STORG_IDENTITY_PUBLIC_URL` | Identity BFF for sign-in nav |
| `STORG_CONTACT_PUBLIC_URL` | Contact service for nav |
| `STORG_CART_PUBLIC_URL` | Cart service for nav |
| `STORG_GITHUB_ORG` | GitHub org to list (default `sigmatactical-org`) |
| `STORG_GITHUB_TOKEN` | Token for the GitHub API; falls back to `GITHUB_TOKEN`. Unset means unauthenticated requests, which work for public data at a lower rate limit |
| `STORG_GITHUB_API_BASE` | GitHub API base URL, for pointing tests at a stub (default `https://api.github.com`) |

## GitHub metadata

Curated repository descriptions, production homepages, and topics live in
`scripts/sync-github-metadata.sh`. Run after editing copy in `src/catalog.rs`:

```bash
./scripts/sync-github-metadata.sh
```

## Local dev

```bash
cargo run
```

### Shared crates

`sigma-theme` and `sigma-pg` are pinned git dependencies, so a
fresh clone builds with nothing but `cargo`: the revision in `Cargo.toml` is
fetched, and `build.rs` writes the `askama.toml` that points at sigma-theme's
templates wherever Cargo put them.

When one of those crates is checked out beside this repo and you are editing it,
link the checkouts so your edits are picked up without a push:

```bash
./scripts/prepare-local.sh
```

That writes `[patch]` entries into `.cargo/config.toml` (gitignored) for the
crates it finds and leaves the rest on their pinned revision; it prints what it
linked. Undo by deleting the file. Note that building against a linked checkout
rewrites `Cargo.lock` into path form — don't commit that; `platform`'s
`scripts/relock.sh` restores the git-resolved lockfile CI expects.

Bumping a shared crate is `platform/scripts/pin-shared-revs.sh <crate>` after
that crate is pushed, which updates every consumer's pin at once.

## Docker

```bash
./scripts/docker-build.sh
docker build -f Dockerfile build/image
```

## Copyright

© Sigma Tactical Group. All rights reserved.

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

Licensed under either of the Apache License, Version 2.0 or the MIT license, at your option.
