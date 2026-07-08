# sigmatactical.org

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

Curated repository descriptions (and optional homepages) live in
`scripts/sync-github-descriptions.sh`. Run after editing copy in `src/catalog.rs`:

```bash
./scripts/sync-github-descriptions.sh
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
