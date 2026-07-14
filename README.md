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

## GitHub metadata

Curated repository descriptions, production homepages, and topics live in
`scripts/sync-github-metadata.sh`. Run after editing copy in `src/catalog.rs`:

```bash
./scripts/sync-github-metadata.sh
```

## Local dev

```bash
./scripts/prepare-local.sh
cargo run
```

Expects sibling `../theme` in the monorepo, or clones theme in CI.

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
